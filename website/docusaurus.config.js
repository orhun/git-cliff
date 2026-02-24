// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer").themes.github;
const darkCodeTheme = require("prism-react-renderer").themes.dracula;

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
  markdown: {
    hooks: {
      onBrokenMarkdownLinks: 'warn',
    },
  },

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  themes: [
    [
      require.resolve("@easyops-cn/docusaurus-search-local"),
      ({
        // `hashed` is recommended as long-term-cache of index file is possible.
        hashed: true,
      }),
    ],
  ],

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          editUrl: "https://github.com/orhun/git-cliff/tree/main/website/",
        },
        blog: {
          onUntruncatedBlogPosts: "ignore",
          showReadingTime: true,
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
          { to: "/blog", label: "News", position: "left" },
          {
            label: "💖 Sponsor",
            href: "https://github.com/sponsors/orhun",
            position: "right",
          },
          {
            href: "https://github.com/orhun/git-cliff",
            "aria-label": "GitHub",
            className: "header-github-link",
            position: "right",
          },
          {
            href: "https://discord.gg/W3mAwMDWH4",
            "aria-label": "Discord server",
            position: "right",
            className: "header-discord-link",
          },
        ],
      },
      announcementBar: {
        content: `⛰️ <b><a target="_blank" href="https://git-cliff.org/blog/2.12.0">git-cliff v2.12.0</a> is now out!</b> 🥳️`,
        backgroundColor: "#243840",
        textColor: "#ffffff",
        isCloseable: true,
      },
      colorMode: {
        defaultMode: "dark",
        disableSwitch: false,
        respectPrefersColorScheme: false,
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
              {
                label: "News",
                to: "/blog",
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
                label: "GitHub",
                href: "https://github.com/orhun/git-cliff",
              },
              {
                label: "Mastodon",
                href: "https://fosstodon.org/@git_cliff",
              },
              {
                label: "Twitter",
                href: "https://twitter.com/git_cliff",
              },
            ],
          },
        ],
        copyright: `Copyright © 2021-${new Date().getFullYear()} git-cliff.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: [
          "bash",
          "diff",
          "json",
          "yaml",
          "toml",
          "rust",
          "markdown",
        ],
      },
    }),
};

module.exports = config;
