<div class="container">
	<h2>Hello world! 👋</h2>
	<h3>My name is Karol Moroz.<br />I am a software developer based in Poland.</h3>
	<section class="ctas">
		<a href="/blog">Read my blog</a>
		<a href="https://github.com/moroz">Follow me on Github</a>
	</section>
</div>

<style lang="scss">
	h2 {
		font-size: 7rem;
		margin-top: 5rem;
	}

	h3 {
		font-size: 4rem;
		font-weight: 400;
	}

	.ctas {
		margin-top: 1.5rem;
		flex-direction: column;
		align-items: flex-start;
		gap: 1rem;
		display: flex;
	}

	a {
		color: inherit;
		text-transform: uppercase;
		display: inline-block;
		font-size: 1.25rem;
		margin-bottom: 0.75em;
		text-decoration: none;
		position: relative;
		padding: 0.5em;
		margin-left: -0.5em;

		&::after {
			content: "";
			position: absolute;
			bottom: 0;
			left: 0.5em;
			right: 0.5em;
			background-color: currentColor;
			height: 1px;
		}

		&:hover {
			&::after {
				background-color: var(--primary);
			}
		}
	}
</style>
