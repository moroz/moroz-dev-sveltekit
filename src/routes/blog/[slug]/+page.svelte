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

    :global(figure) 
      display: flex
      flex-direction: column
      align-items: center
      margin: 1.25em 0
    
      :global(img)
        max-width: 100%
        max-height: 60vh

        @include light 
          outline: 1px solid #aaa
      
    :global(figcaption)
      text-align: center
      font-size: 0.9em
      counter-increment: figure

      &::before 
        content: "Fig. " counter(figure) ": "
</style>
