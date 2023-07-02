<script lang="ts">
	import type { BasicPostData } from "@api/interfaces";
	import BlogEntry from "./BlogEntry.svelte";
	import Pagination from "@components/blog/Pagination.svelte";

	export let posts: BasicPostData[];
	export let totalPages: number;
	export let currentPage: number;
</script>

<div class="container">
	<h2>Blog</h2>
	{#each posts as post}
		<BlogEntry {post} />
	{/each}
	<Pagination {currentPage} {totalPages} />
</div>
