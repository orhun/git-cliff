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
