<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  export let using_workaround: boolean = false;

  export async function shell_open(e) {
    e.preventDefault();
    await invoke("shell_open");
  }

  export async function toggle_workaround(e) {
    e.preventDefault();
    const target = Boolean(e.target.checked);
    console.log("Setting workaround", target);
    using_workaround = await invoke("set_workaround", { active: target });
  }
</script>

<main>
  <h1>Issue #4526</h1>
  <p>
    While this doesn't address the issue, it should demonstrate the cause.
  </p>
  <div class="focus {using_workaround ? 'green' : 'red'}">
    <p>
      <label>
        <input
          type="checkbox"
          checked={using_workaround}
          on:click={toggle_workaround}
        />
        Enable workaround
      </label>
    </p>
    <ul class="left">
      <li>
        <a
          href="https://github.com/tauri-apps/tauri/issues/4526#workaround={using_workaround}"
          target="_blank"
        >
          A <code>target="_blank"</code> link
        </a>
      </li>
      <li>
        <a href="#" on:click={shell_open}>
          Open URL with <code>shell::open()</code>
        </a>
      </li>
    </ul>
  </div>
  <details>
    <summary>The details</summary>
    <p>
      It looks like an <code>xdg-open</code> call starts a new process from within the AppImage runtime.
      By touching a <code>$XDG_RUNTIME_DIR/flatpak-info</code> file, we'll trick it into using "portals"
      (<code>org.freedesktop.portal.OpenURI</code> dbus message).
    </p>
  </details>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }

  .focus {
    display: inline-block;
    margin: 2em auto;
    font-size: large;
    line-height: 2em;
    padding: 2em;
  }

  .left {
    text-align: left;
  }

  .red {
    background-color: #fee;
  }
  .green {
    background-color: #efe;
  }
</style>
