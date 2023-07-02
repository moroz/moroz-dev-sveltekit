<script lang="ts">
	import type { BasicPostData } from "@api/interfaces";
	import type { PageData } from "./$types";
	import BlogPage from "@components/blog/BlogPage.svelte";

	export let data: PageData & { posts: BasicPostData[]; totalPages: number; currentPage: number };
	const { totalPages, posts, currentPage } = data;
</script>

<BlogPage {totalPages} {posts} {currentPage} />
