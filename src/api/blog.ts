import fs from "fs";
import matter from "gray-matter";
import { join, basename } from "path";
import type { BasicPostData, BlogEntry, Post, Video } from "@api/interfaces";
import dayjs from "dayjs";
import * as glob from "glob";
import { formatMarkdown } from "$lib/markdown";

const isDev = process.env.NODE_ENV !== "production";

const postsDirectory = join(process.cwd(), "src/content/blog");
const videosDirectory = join(process.cwd(), "src/content/videos");

export const POSTS_PER_PAGE = 20;

export async function getPostDataBySlug(slug: string) {
	const path = glob.sync(`${postsDirectory}/**/${slug}.md`)[0];
	return getPostData({ slug, filename: path });
}

export async function buildMetadata(data: Record<string, any>) {
	const date = new Date(data.date).toISOString();
	const datePretty = dayjs(date).format("MMMM D, YYYY");
	const summary = data.summary ? await formatMarkdown(data.summary) : null;
	const summaryPlain = data.summary ?? null;
	return {
		date,
		datePretty,
		summary,
		summaryPlain,
		slug: data.slug,
		title: data.title,
		lang: data.lang ?? "en",
		draft: data.draft ?? false,
	};
}

export function readAndParseMarkdownFile(filename: string) {
	const fileContents = fs.readFileSync(filename, "utf8");
	return matter(fileContents);
}

export async function getPostData({ filename: filename }: BlogEntry): Promise<Post> {
	const { data, content } = readAndParseMarkdownFile(filename);
	const metadata = await buildMetadata(data);
	return {
		...metadata,
		content,
		filename,
	};
}

export function getBasicPostData({ filename }: BlogEntry): Promise<BasicPostData> {
	const { data } = readAndParseMarkdownFile(filename);
	return buildMetadata(data);
}

export function getVideoData({ filename }: BlogEntry): Video {
	const fileContents = fs.readFileSync(filename, "utf8");
	const parsed = matter(fileContents);
	const { data, content } = parsed;
	const date = new Date(data.date).toISOString();
	const datePretty = dayjs(date).format("MMMM D, YYYY");
	return {
		slug: data.slug,
		title: data.title,
		date,
		datePretty,
		content,
		youtube: data.youtube,
		filename,
	};
}

export function getAllSlugs(directory: string) {
	return glob.sync(`${directory}/**/*.md`).map((filename) => {
		const slug = basename(filename, ".md");
		return {
			slug,
			filename,
		};
	});
}

export function getAllPostSlugs() {
	return getAllSlugs(postsDirectory);
}

export function getAllVideoSlugs() {
	return getAllSlugs(videosDirectory);
}

export async function getAllPostData() {
	return await Promise.all(getAllSlugs(postsDirectory).map(getBasicPostData));
}

export function getAllVideoData() {
	return getAllSlugs(videosDirectory).map(getVideoData);
}

export async function getSortedPostData() {
	return (await getAllPostData())
		.filter((post) => isDev || !post.draft)
		.sort((a, b) => {
			if (a.date > b.date) return -1;
			if (a.date < b.date) return 1;
			if (a.slug > b.slug) return -1;
			if (a.slug < b.slug) return 1;
			return 0;
		});
}

export function getSortedVideoData() {
	return getAllVideoData().sort((a, b) => {
		if (a.date > b.date) return -1;
		if (a.date < b.date) return 1;
		return 0;
	});
}

export async function paginatePosts(page = 1): Promise<[BasicPostData[], number]> {
	const allPosts = await getSortedPostData();
	const totalPages = Math.ceil(allPosts.length / POSTS_PER_PAGE);
	const startOffset = (page - 1) * POSTS_PER_PAGE;
	const paginated = (await getSortedPostData()).slice(startOffset, startOffset + POSTS_PER_PAGE);
	return [paginated, totalPages];
}
