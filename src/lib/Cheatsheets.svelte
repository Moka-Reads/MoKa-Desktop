<script>
  import { invoke } from "@tauri-apps/api/tauri";

  async function fetch_cheatsheets() {
    let data = await invoke("fetch_cheatsheets");
    let cheatsheets = JSON.parse(data);
    return cheatsheets;
  }

  let content = "";
  let icon = "";
  let title = "";
  let author = "";
  let lang = "";

  let cheatsheets = fetch_cheatsheets();
  let show_home = true;

  function view_content(c, i, t, a, l) {
    content = c;
    icon = i;
    title = t;
    author = a;
    lang = l;
    show_home = false;
  }
</script>

<svelte:head>
  <link href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-okaidia.min.css" rel="stylesheet" />
</svelte:head>


{#await cheatsheets}
  Waiting...
{:then result}
  {#if show_home}
    <div class="cheatsheets-container">
      <div class="language-group">
        <h2><i class="devicon-rust-plain" /> Rust</h2>
        <ul class="cheatsheets-grid">
          {#each result["Rust"] as r}
            <li><button class="w3-button w3-round" on:click={() => view_content(r.content, r.metadata.icon, r.metadata.title, r.metadata.author,r.metadata.lang)}>{r.metadata.title}</button></li>
          {/each}
        </ul>
      </div>
      <!--


  <div class="language-group">
      <h2><i class="devicon-kotlin-plain"></i> Kotlin</h2>
      <ul class="cheatsheets-grid">
          {% for k in kotlin %}
          <li><a href="/cheatsheets/kotlin/{{k.slug}}">{{k.metadata.title}}</a></li>
          {% endfor %}
      </ul>
  </div>
  <div class="language-group">
      <h2><i class="devicon-c-plain"></i> C</h2>
      <ul class="cheatsheets-grid">
          {% for v in c %}
          <li><a href="/cheatsheets/c/{{v.slug}}">{{v.metadata.title}}</a></li>
          {% endfor %}
      </ul>
  </div>
  <div class="language-group">
      <h2><i class="devicon-cplusplus-plain"></i> C++</h2>
      <ul class="cheatsheets-grid">
          {% for v in cpp %}
          <li><a href="/cheatsheets/c++/{{v.slug}}">{{v.metadata.title}}</a></li>
          {% endfor %}
      </ul>
  </div>
  <div class="language-group">
      <h2><i class="devicon-zig-original"></i> Zig</h2>
      <ul class="cheatsheets-grid">
          {% for z in zig %}
          <li><a href="/cheatsheets/zig/{{z.slug}}">{{z.metadata.title}}</a></li>
          {% endfor %}
      </ul>
  </div>
  <div class="language-group">
      <h2><i class="devicon-python-plain"></i> Python</h2>
      <ul class="cheatsheets-grid">
          {% for p in python %}
          <li><a href="/cheatsheets/python/{{p.slug}}">{{p.metadata.title}}</a></li>
          {% endfor %}
      </ul>
  </div>
  <div class="language-group">
      <h2><i class="devicon-swift-plain"></i> Swift</h2>
      <ul class="cheatsheets-grid">
          {% for s in swift %}
          <li><a href="/cheatsheets/swift/{{s.slug}}">{{s.metadata.title}}</a></li>
          {% endfor %}
      </ul>
  </div>
  <div class="language-group">
      <h2><i class="devicon-go-plain"></i> Go</h2>
      <ul class="cheatsheets-grid">
          {% for g in go %}
          <li><a href="/cheatsheets/go/{{g.slug}}">{{g.metadata.title}}</a></li>
          {% endfor %}
      </ul>
  </div>
  <div class="language-group">
      <h2><i class="devicon-github-original"></i> Other</h2>
      <ul class="cheatsheets-grid">
          {% for o in other %}
          <li><a href="/cheatsheets/other/{{o.slug}}">{{o.metadata.title}}</a></li>
          {% endfor %}
      </ul>
  </div>
  -->
    </div>
  {:else}
  <div class="container w3-mobile">

    <div class="article-content w3-mobile">
        <section class="cheatsheet-section w3-mobile">
            <i class="language-icon {icon}"></i>
            <h1 class="title">{title}</h1>
            <h2>By: {author}</h2>
            <h3> Language: {lang.replace(lang[0], lang[0].toUpperCase())}</h3>

            <div class="article-content w3-mobile">
                {@html content}
            </div>
        </section>
    </div>
</div>
  {/if}
{:catch error}
  error: {error}
{/await}
