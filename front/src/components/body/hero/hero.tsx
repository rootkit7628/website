import { component$ } from "@builder.io/qwik";
import styles from "./hero.module.css";
import CyberPunckVideo from "../../../media/cyberpunck.mp4";


export default component$(() => {
  return (
    <div class={["container", styles.hero]}>
      <video autoplay={true} loop muted playsInline class={styles.video}>
        <source src={CyberPunckVideo} type="video/mp4" />
      </video>
      <h1>
        Arlème <span class="highlight">Johnson</span>
      </h1>
      <p>Python Developers | Devops Engineer</p>
      <div class={styles["button-group"]}>
        <button
          onClick$={async () => {
            const defaults = {};
          }}
        >
          My Story
        </button>
        <a
          href="https://qwik.builder.io/docs"
          target="_blank"
          class="button button-dark"
        >
          Want hire me ?
        </a>
      </div>
    </div>
  );
});
