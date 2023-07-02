import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkSmartypants from "remark-smartypants";
import rehypeStarryNight from "./rehype-starry-night";
import rehypeStringify from "rehype-stringify";
import remarkRehype from "remark-rehype";

export async function formatMarkdownWithoutSyntax(md: string) {
	const processed = await unified()
		.use(remarkParse)
		.use(remarkSmartypants)
		.use(remarkRehype)
		.use(rehypeStringify)
		.process(md);

	return String(processed);
}

export async function formatMarkdown(md: string) {
	const processed = await unified()
		.use(remarkParse)
		.use(remarkSmartypants)
		.use(remarkRehype, { allowDangerousHtml: true })
		.use(rehypeStarryNight)
		.use(rehypeStringify, { allowDangerousHtml: true })
		.process(md);

	return String(processed);
}
