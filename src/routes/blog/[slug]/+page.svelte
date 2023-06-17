<script lang="ts">
	import type { Post } from "@api/interfaces";

	export let data: { post: Post };
	const post = data.post;
</script>

<div class="container">
	<h2>{post.title}</h2>

	<div class="content system-font">
		{@html post.content}
	</div>
</div>
