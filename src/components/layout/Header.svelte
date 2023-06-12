<header>
	<div class="container">
		<a class="logo" href="/">
			<h1>moroz.dev</h1>
		</a>

		<nav>
			<a href="/">Home</a>
			<a href="/blog">Blog</a>
			<a href="/contact">Contact</a>
			<a href="/videos">Videos</a>
			<a href="/resume.pdf">Résumé</a>
		</nav>
	</div>
</header>

<style lang="scss">
	@import "@css/prelude";

	.container {
		display: flex;
		height: $header-height;
		overflow: hidden;
		align-items: center;
		justify-content: space-between;
	}

	a {
		text-decoration: none;
		color: inherit;
	}

	h1 {
		line-height: $header-height;
		margin: 0;
	}

	nav {
		display: flex;
		font-weight: 500;
		font-size: 1.125rem;

		a {
			padding: 0 1em;
			height: $header-height;
			line-height: $header-height;

			&:hover {
				background: var(--primary);
				color: var(--body-bg);
			}
		}
	}
</style>
