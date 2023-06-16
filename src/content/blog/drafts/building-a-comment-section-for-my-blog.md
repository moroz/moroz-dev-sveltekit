---
title: Building a Comment Section for My Blog
date: 2021-11-19
slug: building-a-comment-section-for-my-blog
draft: true
tags:
  - Elixir
  - React
  - Next.js
summary: |
  How I built the new comment section for this website using Elixir, Phoenix, Absinthe GraphQL, and Next.js.
---

## Introduction

Static websites are getting increasingly popular these days. There are dozens of very good static site generators
out there, the most popular being [Next.js](https://nextjs.org/), [Hugo](https://gohugo.io/), and [Gatsby](https://www.gatsbyjs.com/).
Static websites have numerous advantages over dynamically generated websites. Most importantly, they are very fast
and cheap to deploy: once your generator spits out a bunch of HTML, CSS, and JS files, you can host the whole website
using very simple hosting solutions, such as [AWS S3](https://aws.amazon.com/s3/), and even deploy it for free
to [Netlify](https://www.netlify.com/) in a few clicks. This website, too, is [written in Next.js](https://github.com/moroz/moroz.dev)
and hosted on Netlify.

The only downside to static websites is that, well, they are static. Once generated and deployed, there is no simple way
to add any dynamic content to the website. But even though you cannot modify the static HTML that gets sent over the wire,
you can still change things in the browser using JavaScript.

Some static site generators, such as Next.js, Gatsby, and [Nuxt](https://nuxtjs.org/), are built around client-side
JavaScript frameworks, such as [React.js](https://reactjs.org/) and [Vue](https://vuejs.org/).
Using these frameworks, you can easily extend the static parts of the website with dynamic elements, such as a shopping cart,
a chat widget, or a fancy animation.

## Inspiration

I have recently seen the article [Various ways to include comments on your static site](https://darekkay.com/blog/static-site-comments/)
by Darek Kay on the main page of Hacker News.
The article describes several solutions to the problem of implementing comments on static websites.
Considering how I got around 15 comments over roughly 13 years of blogging, I decided to invest some 
