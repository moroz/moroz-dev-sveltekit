---
title: Building a login page with cycling backgrounds
author: "Karol Moroz <k.j.moroz@gmail.com>"
date: 2021-09-17
slug: optimizing-background-transitions-on-login-page
draft: true
---

# What this is about

I have recently worked on a login page for a Web application that featured full-screen background images on the login page.
There were 12 different images, their order randomized on the back end on every request.
10 seconds after the first render, the first image would fade out, and the next image would fade in, cycling through all the images at an interval.

The initial design of that page loaded all the images at once, and the transition was implemented as a CSS animation (rather than a CSS transition).
Each image was fading in the same way, but with a different `animation-delay`, so that only one image was showing at a given time.
This first iteration was looking as intended but the page proved very slow to load once deployed to staging.
The image files themselves totaled about 27 MB, all of which had to be fetched on page load and were not properly cached.
Needless to say, Google Lighthouse wasn't happy, giving the page a very low score.

# Proposed solution

In order to keep loading times reasonable, the view needed to be rewritten in such a way that would **minimize the amount of data** that had to be downloaded before the first render.
During the first 10 seconds on this page, the only things that the browser really needs are the HTML markup, the first background image, and the usual CSS and JS bundles.
The other backgrounds would then be **fetched one at a time, on an as-needed basis**.
Other possible optimizations include: **compressing and re-encoding the images** (as JPEG and WEBP) to make them as lightweight as possible, and **setting proper cache headers** for image files to instruct the browser to cache them indefinitely.

<!-- In this article I would like to elaborate on these two steps. -->

# Creating a new project

We will implement the whole view from scratch, writing all the necessary CSS and JavaScript from scratch.
For the development server, I will use a fairly minimal setup of [Vite.js](https://vitejs.dev/) with SASS and Pug.
First, let's create a Vite project, install dependencies using yarn, and initialize a Git repository in the folder:

```bash
yarn create vite --template vanilla-ts dynamic-bg
cd dynamic-bg
yarn
git init
git add -A
git commit -m "Initial commit"
```

Now, let's install SASS and Pug to simplify writing CSS and HTML:

```bash
yarn add sass pug vite-plugin-pug
```

We need to configure `vite-plugin-pug` to instruct Vite on how to process Pug templates.
Create a file called `vite.config.ts` in the root folder of the repository with the following content:

```typescript
import { defineConfig } from "vite";
import pugPlugin from "vite-plugin-pug";

export default defineConfig({
  plugins: [pugPlugin()]
});
```

Now we will need some nice background images.
For the purposes of this exercise, I hand-picked twelve images of Taiwan, a tropical island on the Pacific Ocean.
These images came from
[here](https://unsplash.com/photos/phjpsNFw6pM), [here](https://unsplash.com/photos/5K_C8tRn678),
[here](https://unsplash.com/photos/BNZyrmMV51c), [here](https://unsplash.com/photos/JgFTqBHIHFE),
[here](https://unsplash.com/photos/Q7ssPp8bPtw), [here](https://unsplash.com/photos/cVHqIdkxE7U),
[here](https://unsplash.com/photos/oc2hzcGLCFU), [here](https://unsplash.com/photos/UGoUK9Ev7jU),
[here](https://unsplash.com/photos/NWkztkVe1HU), [here](https://unsplash.com/photos/DGLYo2q2gj8),
[here](https://unsplash.com/photos/TWxB8pT5eHs), and [here](https://unsplash.com/photos/TWxB8pT5eHs).
I assigned 1-indexed numbers to them and placed them in the project's `public` directory, which is the place to put static content in Vite.

# 1st iteration: Markup and cycling backgrounds

Now let's start coding our login page. First, create a directory at `src/css` and inside it, an empty file called `main.sass`. Then, in `src/main.ts`, replace all existing code with a single line that imports the file we just created:

```typescript
import "./css/main.sass";
```

This will load and process the SASS files the project and apply styling on our views.

In `index.html`, replace all existing markup for page content with a slot for a Pug template:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="favicon.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Vite App</title>
  </head>
  <body>
    <pug src="login.pug" /><!-- Add this line -->
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

Create a file called `login.pug` in the same directory and add the following markup:

```pug
.dynamic-bgs#dynamic-bgs
  - for (let i = 1; i <= 12; i++)
    picture
      img(src=`/${i}.jpg` alt="")
```

This will create 12 `<picture>` tags with nested `<img>`s. At this point, the outer `<picture>` tag is not necessary, but we will need it later on to provide fallback images for browsers that don't support WEBP, such as Safari up to version 13.

If we launch `yarn dev` right now, we will see the images properly loaded, displayed all at once on a single page. Let's add some styles to position them:

```sass
*
  box-sizing: border-box

html
  font-size: 100%

body
  font-family: system-ui
  padding: 0
  margin: 0
  height: 100vh
  overflow: hidden

.dynamic-bgs
  picture
    display: block

    img
      position: fixed
      top: 50%
      right: 0
      left: 0
      min-width: 100%
      transform: translateY(-50%)
```

At this point, the background should fill the whole page, and only one image should be displaying at a time.
In the next step, we will write some JavaScript to periodically toggle background images.
Let's start from a bit of placeholder code. Create a file called `src/js/DynamicBackgrounds.ts`:

```typescript
// Backgrounds change every 10 seconds
const CHANGE_INTERVAL = 10000;

export default function DynamicBackgrounds() {
  const container = document.getElementById("dynamic-bgs");
  if (!container) return false;

  // ... logic goes here

  return true;
}
```

In `src/main.ts`, import and call this function:

```typescript
import "./css/main.sass";
import DynamicBackgrounds from "./js/DynamicBackgrounds";

DynamicBackgrounds();
```

By putting this logic in a separate module, we can easily group code by its purpose in the application.
If at a later stage we decided to get rid of this feature, the only thing we would have to do would be commenting out the two lines we just added.

Let us quickly go over the few lines of code that we put in the `DynamicBackgrounds` function:

```typescript
const container = document.getElementById("dynamic-bgs");
if (!container) return false;
```

First, we check for the existence of an element with the ID of `dynamic-bgs` in the DOM. This is the parent container of all `<picture>` tags that
we put in `login.pug`. If this tag does not exist in the DOM (for example, because the user is currently on a different page in the application),
the function returns `false` and no further code is executed.

```typescript
// ... logic goes here
return true;
```

If, however, the container is in place, we can proceed to perform other actions, and we return
`true`. With this simple boolean return value, we can easily check from another module whether the feature has been used on the page or not.

In the same file, above the default export, add a helper function that searches for the background with a given number:

```typescript
function findPictureByIndex(index: number) {
  return document.querySelector(`#dynamic-bgs picture:nth-child(${index})`);
}
```

This function is a thin wrapper over browser built-ins, leveraging CSS selectors to find a specific element in the DOM.
[Document.querySelector](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector) returns the first element matching a given CSS selector string, or `null` if none was found.
The selector `#dynamic-bgs picture` means &ldquo;a `<picture>` element that is located inside the element with the ID `dynamic-bgs`.&rdquo;
The [`:nth-child`](https://developer.mozilla.org/en-US/docs/Web/CSS/:nth-child) selector used here points to the element with at the given index inside a group.
The numbers are 1-indexed, so the first image will have the number 1.
The `index` parameter is injected into the selector using [expression interpolation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#expression_interpolation) syntax, therefore the string literal must be surrounded by backticks (<code>&#96;</code>) rather than single or double quotes.

The helper function `findPictureByIndex` is then used in another helper function:

```typescript
function toggleBg(index: number) {
  const target = findPictureByIndex(index);
  const current = document.querySelector("#dynamic-bgs picture.active");

  if (!target || target.isSameNode(current)) return;
  current?.classList.remove("active");
  target.classList.add("active");
}
```

This function applies the CSS class `.active` on the background picture with the given index and removes that class from any other background picture that might be there.
If there is no picture with the given index, or if the active image is the same one as the target image, we do not perform any modifications to the DOM, otherwise we remove the `.active` class from the active element and apply that class on the target element.

Back in the main `DynamicBackgrounds` function, we can add the remaining code to periodically toggle backgroud images:

```typescript
export default function DynamicBackgrounds() {
  const container = document.getElementById("dynamic-bgs");
  if (!container) return false;

  const imageCount = container.querySelectorAll("picture").length;

  const scheduleBackgroundChange = (index: number) => {
    if (index > imageCount) {
      scheduleBackgroundChange(1);
      return;
    }

    toggleBg(index);
    setTimeout(() => scheduleBackgroundChange(index + 1), CHANGE_INTERVAL);
  };

  scheduleBackgroundChange(1);

  return true;
}
```

If you navigate to this page now, you should see 
