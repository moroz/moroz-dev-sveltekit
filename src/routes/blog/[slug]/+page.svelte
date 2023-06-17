<script lang="ts">
	import { formatDate } from "$lib/dateHelpers";

	import type { Post } from "@api/interfaces";

	export let data: { post: Post };
	const post = data.post;
</script>

<div class="container">
	<h2 class="title">{post.title}</h2>
	<section class="meta">
		<p>{formatDate(post.date)}</p>
	</section>

	<div class="content system-font" lang={post.lang}>
		{@html post.content}
	</div>
</div>

<style lang="sass">
  @import @css/prelude

  .title
    font-weight: 700
    font-size: 2rem
    text-align: center

  .meta
    text-align: center

  .content
    font-size: 1.125em
    line-height: 1.5

    :global(h2),
    :global(h3),
    :global(h4)
      font-family: $font-family-sans
      font-weight: 700

    :global(h2)
      font-size: 2rem
</style>
