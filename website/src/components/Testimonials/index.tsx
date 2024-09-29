import React from "react";
import styles from "./styles.module.css";

const testimonials = [
  {
    name: "Boshen",
    handle: "boshen",
    date: "Sep 29, 2024",
    content:
      "git-cliff is amazing. I managed to use its published crates for my project's release management.",
    url: "https://x.com/boshen_c/status/1840391571495420362",
  },
  {
    name: "Greptime Database",
    handle: "GreptimeTeam",
    date: "Apr 9, 2024",
    content:
      "The changelog is powered by git-cliff. Thank you @orhundev for this elegant tool!",
    url: "https://x.com/Greptime/status/1777707539255189544",
  },
  {
    name: "CodeSee.io",
    handle: "Codesee-io",
    date: "Jan 24, 2023",
    content:
      "With git-cliff, you can generate a changelog from your Git history FAST.",
    url: "https://x.com/Codeseeio/status/1617901319179702274",
  },
  {
    name: "GitHub, Inc.",
    handle: "github",
    date: "May 4, 2022",
    content:
      "Whether you use conventional commits or your own conventions, @orhundev’s git-cliff is a great little tool for generating changelogs.",
    url: "https://x.com/github/status/1521943000817057795",
  },
];

const getRandomTestimonial = () => {
  const randomIndex = Math.floor(Math.random() * testimonials.length);
  return testimonials[randomIndex];
};

export default function Testimonials(): JSX.Element {
  const review = getRandomTestimonial();

  return (
    <div className={styles.testimonialsList}>
      <div className={styles.testimonial}>
        <p className={styles.content}>
          <em>"{review.content}"</em>
        </p>
        <div className={styles.details}>
          <span className={styles.author}>
            - {review.name} (
            <a
              href={`https://github.com/${review.handle}`}
              target="_blank"
              rel="noopener noreferrer"
            >{`@${review.handle}`}</a>
            )
          </span>
          <a
            href={review.url}
            target="_blank"
            rel="noopener noreferrer"
            className={styles.date}
          >
            {review.date}
          </a>
        </div>
      </div>
    </div>
  );
}
