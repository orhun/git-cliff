import chalk from "chalk";
import { test, expect, type ConsoleMessage, type Page } from "@playwright/test";

type ReportType = "DocusaurusError" | "ConsoleError" | "ConsoleWarning";

interface ErrorReport {
  type: ReportType;
  message: string;
  reactErrorCode?: string;
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

test("crawl site and detect errors", async ({ page, baseURL }) => {
  test.setTimeout(600000);
  if (!baseURL) throw new Error("baseURL is required");

  const base = normalizeUrl(baseURL);
  const visited = new Set<string>();
  const queue: string[] = [base];
  const errors: ErrorReport[] = [];
  const errorCache = new Set<string>();

  page.on("console", (message: ConsoleMessage) => {
    const text = message.text();
    const lower = text.toLowerCase();
    const url = page.url();

    if (errorCache.has(`${url}:${text}`)) return;
    if (lower.includes("cookie") && lower.includes("samesite")) return;

    const isHydrationError = lower.includes("hydration") || lower.includes("did not match");
    const docusaurusMatch = text.match(/Docusaurus.*:.*Error.*#(\d+)/i);
    let report: ErrorReport | null = null;

    if (docusaurusMatch) {
      report = { type: "DocusaurusError", message: text, reactErrorCode: docusaurusMatch[1], url };
    } else if (lower.includes("docusaurus") && lower.includes("error")) {
      report = { type: "DocusaurusError", message: text, url };
    } else if (isHydrationError || lower.includes("uncaught") || lower.includes("typeerror")) {
      report = { type: "ConsoleError", message: text, url };
    } else if (message.type() === "warning" || lower.includes("warning")) {
      report = { type: "ConsoleWarning", message: text, url };
    }

    if (report) {
      errors.push(report);
      errorCache.add(`${url}:${text}`);
      if (report.type === "DocusaurusError" || report.type === "ConsoleError") {
        logger.error(`${url}: ${report.type}`);
      } else {
        logger.warn(`${url}: ${report.type}`);
      }
    }
  });

  while (queue.length > 0) {
    const url = queue.shift()!;
    if (visited.has(url)) continue;
    visited.add(url);
    logger.info(`Checking: ${url}`);

    for (let attempt = 1; attempt <= 5; attempt++) {
      try {
        await page.goto(url, { waitUntil: "domcontentloaded", timeout: 15000 });
      } catch (e) {
        logger.warn(`Attempt ${attempt} failed for ${url}`);
      }
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

  const criticalErrors = errors.filter(e => e.type !== "ConsoleWarning");
  expect(
    criticalErrors,
    `Found ${criticalErrors.length} errors during crawl:\n${JSON.stringify(criticalErrors, null, 2)}`,
  ).toHaveLength(0);
});
