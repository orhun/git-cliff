import React from "react";
import clsx from "clsx";
import Link from "@docusaurus/Link";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import HomepageFeatures from "@site/src/components/HomepageFeatures";
import Testimonials from "@site/src/components/Testimonials";
import Head from "@docusaurus/Head";
import styles from "./index.module.css";

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className={clsx("hero hero--primary", styles.heroBanner)}>
      <div className="container">
        <div className={styles.bodyContainer}>
          <img className={styles.animContainer} src="/img/git-cliff-anim.gif" />
          <p className="hero__subtitle">{siteConfig.tagline}</p>
          <div className={styles.buttons}>
            <Link className="button button--secondary button--lg" to="/docs/">
              Get Started
            </Link>
          </div>
        </div>
      </div>
    </header>
  );
}

export default function Home(): JSX.Element {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.tagline}`}
      description={`${siteConfig.tagline}`}
    >
      <Head>
        <meta http-equiv="X-UA-Compatible" content="IE=edge" />
        <meta http-equiv="content-type" content="text/html; charset=utf-8" />
        <meta property="og:type" content="website" />
        <meta property="og:url" content={`${siteConfig.url}`} />
        <meta property="og:title" content={`${siteConfig.title}`} />
        <meta property="og:description" content={`${siteConfig.tagline}`} />
        <meta property="og:image" content="/img/git-cliff-social-card.jpg" />
        <meta name="description" content={`${siteConfig.tagline}`} />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <link
          rel="apple-touch-icon"
          sizes="180x180"
          href="/favicon/apple-touch-icon.png"
        />
        <link
          rel="icon"
          type="image/png"
          sizes="32x32"
          href="/favicon/favicon-32x32.png"
        />
        <link
          rel="icon"
          type="image/png"
          sizes="16x16"
          href="/favicon/favicon-16x16.png"
        />
        <link rel="manifest" href="/favicon/site.webmanifest" />
        <script
          async
          src="https://umami.orhun.dev/script.js"
          data-website-id="f75484b0-d2a4-4a0c-a560-cb58080e2f2e"
        ></script>
      </Head>
      <HomepageHeader />
      <main>
        <HomepageFeatures />
        <Testimonials />
      </main>
    </Layout>
  );
}
