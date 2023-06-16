<script lang="ts">
	import type { BasicPostData } from "@api/interfaces";
	import type { PageData } from "./$types";
	import BlogPage from "@components/blog/BlogPage.svelte";

	export let data: PageData & { posts: BasicPostData[]; totalPages: number };
</script>

<BlogPage totalPages={data.totalPages} posts={data.posts} />
