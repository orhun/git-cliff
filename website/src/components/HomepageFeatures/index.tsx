import React from "react";
import clsx from "clsx";
import styles from "./styles.module.css";

type FeatureItem = {
  title: string;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: "Highly Customizable",
    description: (
      <>
        <span className={styles.text}>git-cliff </span>uses
        <b> regex-powered </b>
        custom parsers and the changelog can be customized easily with a
        <b> configuration file </b> to match the desired format.
      </>
    ),
  },
  {
    title: "Conventional Commits",
    description: (
      <>
        <span className={styles.text}>git-cliff </span>can generate changelog
        files for any Git repository that follows the
        <b> conventional commits </b>
        specification.
      </>
    ),
  },
  {
    title: "Easy Integration",
    description: (
      <>
        <span className={styles.text}>git-cliff </span>can be easily integrated
        with your Rust/Python/Node.js project as a<b> command-line tool </b>
        and also can be used as a<b> library </b>
        for Rust projects.
      </>
    ),
  },
];

function Feature({ title, description }: FeatureItem) {
  return (
    <div className={clsx("col col--4")}>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
