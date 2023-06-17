<script lang="ts">
	import type { BasicPostData } from "@api/interfaces";

	export let post: BasicPostData;

	const url = `/blog/${post.slug}`;
</script>

<article class="post">
	<a href={url} class="title">
		<h3>{post.title}</h3>
	</a>
	{#if post.summary}
		<div class="summary">{@html post.summary}</div>
	{/if}
	<a href={url} class="read-more">Read post...</a>
</article>

<style lang="sass">
  @import @css/prelude

  .post
    margin: 1rem 0

    h3
      font-weight: bold

  .title
    color: inherit
    text-decoration: none

  .read-more
    display: block

  .title + .read-more
    margin-top: .5rem
</style>
