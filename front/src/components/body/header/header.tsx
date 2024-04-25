import { component$ } from "@builder.io/qwik";
import styles from "./header.module.css";

export default component$(() => {
  return (
    <header class={styles.header}>
      <div class={["container p-3", styles.wrapper]}>
        <div class={styles.logo}>
          <a href="/" title="qwik">
            <h3>ARLEME</h3>
          </a>
        </div>
        <ul>
          <li>
            <a
              href="https://qwik.builder.io/docs/components/overview/"
              target="_blank"
            >
              Home
            </a>
          </li>
          <li>
            <a
              href="https://qwik.builder.io/examples/introduction/hello-world/"
              target="_blank"
            >
              Services
            </a>
          </li>
          <li>
            <a
              href="https://qwik.builder.io/tutorial/welcome/overview/"
              target="_blank"
            >
              Experiences
            </a>
          </li>
          <li>
            <a
              href="https://qwik.builder.io/docs/overview/"
              target="_blank"
            >
              Formations
            </a>
          </li>
          <li>
            <a
              href="https://qwik.builder.io/docs/overview/"
              target="_blank"
            >
              Contacts
            </a>
          </li>
        </ul>
      </div>
    </header>
  );
});
