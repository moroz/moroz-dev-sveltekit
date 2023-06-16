---
title: "Secure direct upload to S3 using Elixir, Svelte, and Vite"
date: 2021-06-26
slug: secure-direct-upload-to-s3-using-elixir-react-typescript-vite
draft: true
tags:
  - vite
  - s3
summary: |
  This article describes the steps necessary to implement direct uploads
  to AWS S3 in a front-end framework like Svelte or React, secure these
  files using presigned URLs, and display a progress bar.
---

## Introduction

In today's post I would like to discuss the topic of file uploads on the front end.
File uploads are a task that appears in a lot of different use cases, and there
are different approaches to this problem, depending on your requirements.

In some cases, for instance when you are building a social network like Instagram
or Twitter, or when you're building an image gallery, you will most likely want to
upload your files to a bucket or server, process them using a server-side tool or a "serverless"
[Lambda function](https://aws.amazon.com/blogs/compute/resize-images-on-the-fly-with-amazon-s3-aws-lambda-and-amazon-api-gateway/),
and put the converted files behind a CDN service such as [AWS CloudFront](https://aws.amazon.com/cloudfront/) or
[Cloudflare CDN](https://www.cloudflare.com/cdn/).

## Server-side file processing

With regard to server-side tools, you can do everything on the server side that a Linux
machine can do. A common use case is resizing, manipulating, and converting pictures using
[ImageMagick](https://imagemagick.org/index.php).
Those working with movies, videos, and audio may use open-source media codecs such as
[FFmpeg](https://www.ffmpeg.org/).
You can even convert between commonly used office formats, for instance to create
thumbnails of Word documents or Excel spreadsheets, or convert these to PDF using
[unoconv](https://github.com/unoconv/unoconv), which is a command-line wrapper for
[LibreOffice](https://www.libreoffice.org).

In the Elixir ecosystem, there is a library called [Arc](https://github.com/stavro/arc),
which was later forked and the development continued under the name [Waffle](https://github.com/elixir-waffle/waffle).
These libraries handle uploading, transcoding, and deleting files using various storage
providers, and they allow you to execute arbitrary transformations using command-line tools.
Their APIs and documentation are somewhat cryptic, but with some trial and error you can usually get
everything working.
In the Rails ecosystem, there is a tool called [CarrierWave](https://github.com/carrierwaveuploader/carrierwave)
which does the same job and is relatively easier to use than its Elixir counterparts.

## Securing private files in S3 with presigned URLs

Very often when you implement some sort of upload feature in your application, you may want
to keep the files private, and only expose them to authorized users. You can easily achieve
this using [presigned URLs](https://docs.aws.amazon.com/AmazonS3/latest/userguide/ShareObjectPreSignedURL.html).

A presigned URL contains the link to a particular object and only grants the holder access
to that object. You can also set a validity period, so that the URL only stays valid for a given
period of time. Some publishing houses use this technique to let their paying customers download
e-books or other paid content, while others use it to hide confidential information and personal
data from third parties.
