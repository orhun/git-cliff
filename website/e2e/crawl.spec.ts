import chalk from "chalk";
import { test, expect, type ConsoleMessage, type Page } from "@playwright/test";

type ReportType = ReturnType<ConsoleMessage["type"]>;

interface ErrorReport {
  type: ReportType;
  text: string;
  url: string;
}

export const logger = {
  info: (message: string, ...args: any[]) => {
    console.log(chalk.green("[INFO]"), message, ...args);
  },
  warn: (message: string, ...args: any[]) => {
    console.log(chalk.yellow("[WARN]"), message, ...args);
  },
  error: (message: string, ...args: any[]) => {
    console.log(chalk.red("[ERROR]"), message, ...args);
  },
};

const normalizeUrl = (url: string): string => url.replace(/\/+$/, "");

const truncateText = (text: string, max = 128): string =>
  text.length <= max
    ? text
    : text.slice(0, Math.floor(max / 2)) +
      " ... " +
      text.slice(-Math.floor(max / 2));

test("crawl site and detect errors", async ({ page, baseURL }) => {
  test.setTimeout(600000);

  if (!baseURL) {
    throw new Error("baseURL is required");
  }

  const base = normalizeUrl(baseURL);
  const visited = new Set<string>();
  const queue: string[] = [base];
  const reports: Partial<Record<ReportType, number>> = {};

  page.on("console", (message: ConsoleMessage) => {
    const report: ErrorReport = {
      type: message.type(),
      text: message.text(),
      url: page.url(),
    };

    if (report.type === "error" || report.type === "assert") {
      logger.error(`${report.url}: ${truncateText(report.text)}`);
    } else if (report.type === "warning") {
      logger.warn(`${report.url}: ${truncateText(report.text)}`);
    }

    reports[report.type] ??= 0;
    reports[report.type]!++;
  });

  while (queue.length > 0) {
    const url = queue.shift()!;

    if (visited.has(url)) {
      continue;
    } else {
      visited.add(url);
    }

    logger.info(`Checking: ${url}`);
    try {
      await page.goto(url, { waitUntil: "domcontentloaded", timeout: 15000 });
    } catch (e) {
      logger.error(`Failed fetching page for ${url}`);
    }

    const links = await page.$$eval(
      "a[href]",
      (elements, baseUrl) =>
        elements
          .map((a) => (a as HTMLAnchorElement).href.split("#")[0])
          .filter((href) => href.startsWith(baseUrl)),
      base
    );

    for (const link of links) {
      const normalized = normalizeUrl(link);
      if (!visited.has(normalized)) {
        queue.push(normalized);
      }
    }
  }

  const errorCount = (reports.error ?? 0) + (reports.assert ?? 0);

  for (const [type, count] of Object.entries(reports)) {
    logger.info(`${chalk.bgRed(type.toUpperCase())}: ${chalk.bold(count)}`);
  }

  expect(
    errorCount,
    `Found ${errorCount} console errors during crawl:\n${JSON.stringify(reports, null, 2)}`
  ).toBe(0);
});
