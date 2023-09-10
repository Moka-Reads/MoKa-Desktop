<script>
  import Cheatsheets from "./lib/Cheatsheets.svelte";
  let view = "home";

  function home() {
    view = "home";
  }

  function view_cheatsheets() {
    view = "cheatsheet";
  }

  function view_articles() {
    view = "articles";
  }

  function view_guides() {
    view = "guides";
  }

  function view_awesome() {
    view = "awesome";
  }
</script>

<header>
  <h1>MoKa Reads</h1>
</header>

<nav class="w3-bar nav">
  <button on:click={home} class="w3-bar-item w3-button"
    ><i class="fas fa-home" /> Home</button
  >
  <button on:click={view_cheatsheets} class="w3-bar-item w3-button"
    ><i class="fas fa-list-alt" /> Cheatsheets</button
  >
  <button on:click={view_articles} class="w3-bar-item w3-button"
    ><i class="fas fa-newspaper" /> Articles</button
  >
  <button on:click={view_guides} class="w3-bar-item w3-button"
    ><i class="fas fa-info-circle" /> Guides</button
  >
  <button on:click={view_awesome} class="w3-bar-item w3-button"
    ><i class="fas fa-list" /> Awesome Lists</button
  >
</nav>

{#if view === "home"}
  <div class="container">
    <section class="open-source-section">
      <h1>MoKa Reads Desktop</h1>
      <h3>Your Gateway to Diverse Programming Resources</h3>
      <p>
        Welcome to MoKa Reads Desktop, your central hub for a diverse array of
        open-source programming resources. We specialize in several core
        programming languages, each offering unique advantages for developers of
        all levels.
      </p>

      <h2>Resource Variety</h2>

      <p>
        Within MoKa Reads Desktop, you'll discover a rich tapestry of
        programming materials, expertly curated to suit your learning needs:
      </p>

      <ul>
        <li>
          <strong>Cheatsheets</strong>: Quick-reference guides for language
          syntax, libraries, and best practices.
        </li>
        <li>
          <strong>Articles</strong>: In-depth insights, tutorials, and
          real-world use cases.
        </li>
        <li>
          <strong>Guides</strong>: Comprehensive tutorials to master key
          concepts and frameworks.
        </li>
        <li>
          <strong>Awesome Lists</strong>: Curated lists of top resources, tools,
          and libraries.
        </li>
      </ul>

      <h2>Languages in Focus</h2>

      <p>
        MoKa Reads Desktop specializes in the following programming languages,
        each with its own set of advantages:
      </p>

      <ul>
        <li>
          <strong>Rust:</strong> Renowned for its memory safety, Rust empowers developers
          to write fast and secure code. Ideal for systems programming and low-level
          control.
        </li>
        <li>
          <strong>Zig:</strong> Zig combines the best of C and C++ with modern language
          features. Its simplicity and efficiency make it a fantastic choice for
          systems programming.
        </li>
        <li>
          <strong>Python:</strong> Python's readability and versatility make it a
          favorite for web development, data analysis, and automation.
        </li>
        <li>
          <strong>C and C++:</strong> These foundational languages provide high-performance
          capabilities and are essential for system-level programming and game development.
        </li>
        <li>
          <strong>Swift:</strong> Developed by Apple, Swift offers speed and safety
          for iOS and macOS app development.
        </li>
        <li>
          <strong>Go:</strong> Go is known for its efficiency and ease of use, making
          it a great choice for cloud computing and microservices.
        </li>
      </ul>

      <h2>Advantages of Each Language</h2>

      <ul>
        <li>
          <strong>Rust:</strong> Offers strong memory safety guarantees, preventing
          common programming errors.
        </li>
        <li>
          <strong>Zig:</strong> Blends low-level control with modern conveniences,
          making it a compelling choice for system programmers.
        </li>
        <li>
          <strong>Python:</strong> Known for its simplicity, Python is a versatile
          language used in web development, data science, and automation.
        </li>
        <li>
          <strong>C and C++:</strong> Provide high-performance capabilities, allowing
          fine-grained control over hardware resources.
        </li>
        <li>
          <strong>Swift:</strong> Offers speed and safety for iOS and macOS app development,
          with a clear and expressive syntax.
        </li>
        <li>
          <strong>Go:</strong> Known for its efficiency, simplicity, and strong support
          for concurrent programming, making it an excellent choice for building
          scalable systems.
        </li>
      </ul>

      <p>
        MoKa Reads Desktop puts this wealth of knowledge at your fingertips,
        enabling you to explore, learn, and excel in the world of programming.
        Dive into the specific language resources that interest you most and
        take your coding journey to new heights.
      </p>
    </section>
  </div>
{:else if view === "cheatsheet"}
  <Cheatsheets />
{:else}
  <p>Not implemented yet!</p>
{/if}

<style>
  .w3-bar-item {
    width: 20%;
  }
</style>
