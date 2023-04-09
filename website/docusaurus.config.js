// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/vsDark");

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "git-cliff",
  tagline: "A highly customizable changelog generator ⛰️ ",
  favicon: "favicon/favicon.ico",

  // Set the production url of your site here
  url: "https://git-cliff.org",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: "/",

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: "orhun", // Usually your GitHub org/user name.
  projectName: "git-cliff", // Usually your repo name.

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: "https://github.com/orhun/git-cliff/tree/main/website/",
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: "https://github.com/orhun/git-cliff/tree/main/website/",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      image: "img/git-cliff-social-card.jpg",
      navbar: {
        title: "git-cliff",
        logo: {
          alt: "git-cliff logo",
          src: "img/logo.png",
        },
        items: [
          {
            type: "doc",
            docId: "index",
            label: "Documentation",
            position: "left",
          },
          { to: "/blog", label: "Blog", position: "left" },
          {
            label: "GitHub",
            href: "https://github.com/orhun/git-cliff",
            position: "right",
          },
          {
            label: "Sponsor",
            href: "https://github.com/sponsors/orhun",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        links: [
          {
            title: "Documentation",
            items: [
              {
                label: "Get started",
                to: "/docs/",
              },
            ],
          },
          {
            title: "Community",
            items: [
              {
                label: "Discord",
                href: "https://discord.gg/W3mAwMDWH4",
              },
              {
                label: "Matrix",
                href: "https://matrix.to/#/#git-cliff:matrix.org",
              },
            ],
          },
          {
            title: "More",
            items: [
              {
                label: "Blog",
                to: "/blog",
              },
              {
                label: "GitHub",
                href: "https://github.com/orhun/git-cliff",
              },
              {
                label: "Sponsor",
                href: "https://github.com/sponsors/orhun",
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} git-cliff.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ["yaml", "toml", "rust"],
      },
    }),
};

module.exports = config;
