<script lang="ts">
	import clsx from "clsx";

	export let totalPages: number;
	export let currentPage: number;

	const links: [number, string][] = Array.from(new Array(totalPages)).map((_, i) => {
		const url = i === 0 ? "/blog" : `/blog/page/${i + 1}`;
		return [i + 1, url];
	});
</script>

<section class="pagination">
	{#each links as [page, url]}
		<a href={url} data-sveltekit-reload class={clsx(page === currentPage && "active")}>{page}</a>
	{/each}
</section>
