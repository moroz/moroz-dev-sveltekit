import { unified } from "unified";
import remarkHtml from "remark-html";
import remarkParse from "remark-parse";
import remarkSmartypants from "remark-smartypants";

export async function formatMarkdown(md: string) {
	const processed = await unified()
		.use(remarkParse)
		.use(remarkSmartypants)
		.use(remarkHtml)
		.process(md);

	return String(processed);
}
