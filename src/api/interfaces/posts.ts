export interface Post {
	slug: string;
	date: string;
	datePretty: string;
	title: string;
	content: string;
	filename: string;
	lang?: string;
	summary?: string | null;
	summaryPlain: string | null;
	draft: boolean;
}

export interface Video {
	slug: string;
	date: string;
	datePretty: string;
	title: string;
	content: string;
	filename: string;
	youtube: string;
}

export interface BlogEntry {
	slug: string;
	filename: string;
}

export interface BasicPostData {
	slug: string;
	title: string;
	lang: string;
	date: string;
	datePretty: string;
	summary: string | null;
	draft: boolean;
}
