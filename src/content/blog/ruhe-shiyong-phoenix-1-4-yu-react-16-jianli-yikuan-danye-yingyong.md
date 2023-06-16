---
title: 如何使用Phoenix 1.4與React 16建立一款單頁應用
slug: ruhe-shiyong-phoenix-1-4-yu-react-16-jianli-yikuan-danye-yingyong
date: 2018-11-01
description: 如果使用 Phoenix 1.4 框架及 React 16 開發一款簡單的單頁應用程式。
---

### 摘要
本文所介紹的是如何在Phoenix 1.4上建立一個基於React的應用程式。文中所開發的程式極為簡單，也就是一款待辦事清單，只有一個介面。

#### 技術要求
為了完成這篇教程，你需要一臺裝有類UNIX系統（[GNU/Linux](https://www.debian.org/distrib/)、Mac OS X、或[FreeBSD](https://www.freebsd.org/where.html)）的電腦。除此之外，還需要安裝[Erlang/OTP平臺](http://www.erlang.org/downloads)、[Elixir 1.5+](https://elixir-lang.org/install.html#distributions)、[Node.js](https://nodejs.org/en/)與[PostgreSQL資料庫管理系統](https://www.postgresql.org)。安裝以上軟體不在本文章的範圍內。本文假設讀者已有Elixir與JavaScript的基礎（jQuery不算）。

雖然原則上任何編輯器，包括 Microsoft 記事本，都可以寫出良好的軟體，然而這家公司另外開發了一款編輯器，[Visual Studio Code](https://code.visualstudio.com)，筆者推薦寫React的時候都使用這一款編輯器（但我寫其他程式語言都用VIM和Spacemacs）。

[Nigiyaka的Git repo](https://gitlab.com/moroz2137/react-phoenix-workshop-todo-list).

### 建立新的Phoenix應用程式
首先，安裝最新的Phoenix：

```
mix archive.uninstall phx_new
mix archive.install hex phx_new 1.4.0-rc.3
```

然後在一個空的資料夾建立一個Phoenix應用程式。由於筆者認為日文詞彙最適合當作代號，因此決定將這款應用程式命名為 Nigiyaka（賑やか、日文「熱鬧」的意思）。

```
mkdir -p ~/working/react_workshop
cd !$
mix phx.new nigiyaka
```

當Phoenix的代碼生成器問你是否想要安裝依賴的時候（「Fetch and install dependencies?」），我們拒絕，待會在手動安裝依賴。原因是，Phoenix默認採用的Node包管理器為NPM，而本文裏面將會使用另外一款套件管理系統，名叫Yarn。進入新建立的應用程式的資料夾並手動安裝依賴，然後建立 Git repo：

```
cd ~/working/react_workshop/nigiyaka
mix deps.get
cd assets && yarn install
cd ..
git init && git add . && git commit -m "Initial commit"
```

然後建立我們程式專用的資料庫：

```
[~/working/react_workshop/nigiyaka] $ mix ecto.create
The database for Nigiyaka.Repo has been created
```

當我們啟動Phoenix的時候，可見我們的程式同時也會啟動Webpack，那是一款前端打包工具，我們將用它來編譯前端所使用的JS和CSS：

```
[~/working/react_workshop/nigiyaka] $ mix phx.server
[info] Running NigiyakaWeb.Endpoint with cowboy 2.5.0 at http://localhost:4000

Webpack is watching the files…

Hash: ad93202531f9d3e38cc3
Version: webpack 4.4.0
Time: 1123ms
Built at: 11/1/2018 3:37:17 PM
                Asset       Size       Chunks             Chunk Names
       ../css/app.css   10.6 KiB  ./js/app.js  [emitted]  ./js/app.js
               app.js   7.23 KiB  ./js/app.js  [emitted]  ./js/app.js
       ../favicon.ico   1.23 KiB               [emitted]
        ../robots.txt  202 bytes               [emitted]
../images/phoenix.png   13.6 KiB               [emitted]
[./css/app.css] 39 bytes {./js/app.js} [built]
[./js/app.js] 493 bytes {./js/app.js} [built]
   [0] multi ./js/app.js 28 bytes {./js/app.js} [built]
    + 3 hidden modules
Child mini-css-extract-plugin node_modules/css-loader/index.js!css/app.css:
    [./node_modules/css-loader/index.js!./css/app.css] ./node_modules/css-loader!./css/app.css 288 bytes {mini-css-extract-plugin} [built]
    [./node_modules/css-loader/index.js!./css/phoenix.css] ./node_modules/css-loader!./css/phoenix.css 10.9 KiB {mini-css-extract-plugin} [built]
        + 1 hidden module
```

瀏覽至 [http://localhost:4000](http://localhost:4000) 即可看見我們正在運行的程式：

[[Image:210-fig1-new-phoenix-app]]

Phoenix伺服器可以先關掉，接下來我們需要設定React、Webpack和SASS。

### 設定Webpack + React + SASS

```
cd assets
yarn add node-sass sass-loader style-loader
```

Phoenix所生成的 `assets/webpack.config.js` 如下：

```javascript
const path = require('path');
const glob = require('glob');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const UglifyJsPlugin = require('uglifyjs-webpack-plugin');
const OptimizeCSSAssetsPlugin = require('optimize-css-assets-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = (env, options) => ({
  optimization: {
    minimizer: [
      new UglifyJsPlugin({ cache: true, parallel: true, sourceMap: false }),
      new OptimizeCSSAssetsPlugin({})
    ]
  },
  entry: {
      './js/app.js': ['./js/app.js'].concat(glob.sync('./vendor/**/*.js'))
  },
  output: {
    filename: 'app.js',
    path: path.resolve(__dirname, '../priv/static/js')
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader'
        }
      },
      {
        test: /\.css$/,
        use: [MiniCssExtractPlugin.loader, 'css-loader']
      }
    ]
  },
  plugins: [
    new MiniCssExtractPlugin({ filename: '../css/app.css' }),
    new CopyWebpackPlugin([{ from: 'static/', to: '../' }])
  ]
});
```

將這一段：

```javascript
# 31行至34行
{
  test: /\.css$/,
  use: [MiniCssExtractPlugin.loader, 'css-loader']
}
```

改寫成以下的模樣：

```javascript
{
  test: /\.(css|scss|sass)$/,
  exclude: /node_modules/,
  use: ['style-loader', MiniCssExtractPlugin.loader, 'css-loader', 'sass-loader']
}
```

由於我們改了Webpack的設置，必須重新開啟 `mix phx.server` 以重新開啟Webpack。網頁還沒有變，因為CSS檔案還是一樣的。讓我們將 `assets/css/app.css` 改名為 `assets/css/app.sass`。讀者若不習慣寫SASS也可以用 `.scss` 副檔名：

```
git mv assets/css/app.css assets/css/app.sass
```

現在在Phoenix log當中可以看錯誤訊息：

```
ERROR in ./node_modules/mini-css-extract-plugin/dist/loader.js!./node_modules/css-loader!./node_modules/sass-loader/lib/loader.js!./css/app.css
Module build failed: ModuleBuildError: Module build failed: Error: ENOENT: no such file or directory, open '/Users/karol/working/react_workshop/nigiyaka/assets/css/app.css'
```

原因是 `assets/js/app.js` 檔案中，我們引入了 `app.css` 檔案，請進行以下更改：

```javascript
// We need to import the CSS so that webpack will load it.
// The MiniCssExtractPlugin is used to separate it out into
// its own CSS file.
import css from "../css/app.sass" // 以前是 "../css/app.css"
```

現在可以在 `assets/css/app.sass` 使用比較簡縮的語法寫我們的CSS：

```sass
/* This file is for your main application css. */

@import "./phoenix.css"

body
  background-color: darken(Salmon, 10%) !important
  color: yellow !important
```

切換至瀏覽器就可以看見我們的CSS已更新：

[[Image:210-fig2-dark-salmon-bg-yellow-fg]]

現在是儲存更改的好機會：

```
git add -A
git commit -m "Set up SASS"
```

### 迭代0：設計一個模擬介面

雖然我們的程式已經可以改變介面的顔色，然而在`app.sass`寫所有的樣式不是好習慣，比較好的方法是建立專用資料夾：

```
cd assets/css
touch _variables.sass
mkdir base
touch base/{base,import,layout,todo_list}.sass
```

刪除剛剛在`assets/css/app.sass`添加的幾行並引入新建立的檔案。另外載入一種好看的字體：

```sass
/* This file is for your main application css. */

@import url('https://fonts.googleapis.com/css?family=PT+Sans:400,700&subset=latin-ext')

@import "./base/import"
```

在`assets/css/base/import.sass`中引入該資料夾所新增的樣式：

```sass
@import "./base"
@import "./layout"
@import "./todo_list"
```

在`assets/css/_variables.sass`設定幾個變數，待會要用：

```sass
$body-bg: #fff
$text-color: #282828

$container-width: 25rem

$family-latin: 'PT Sans'
$family-cjk: "Microsoft JhengHei", "微軟正黑體", 'Hiragino Sans GB W3', 'Noto Sans CJK TC', 'Noto Sans CJK SC', 'Noto Sans CJK JP' 'Microsoft YaHei', 'Droid Sans Fallback'
$font-sans: #{$family-latin}, #{$family-cjk}, sans-serif
$font-primary: $font-sans

$medium-up: "screen and (min-width: 640px)"
$full-hd-up: "screen and (min-width: 1920px)"

$focus-bg: darken(#fff, 4%)
$todo-font-size: 1.2rem
$todo-padding-left: 1rem
$todo-padding-vertical: 0.75rem
```

在`assets/css/base/base.sass`設定字體、文字大小與顔色：

```sass
@import "../variables"

html
  font-size: 18px
  @media #{$medium-up}
    font-size: 19px
  @media #{$full-hd-up}
    font-size: 20px

body
  background-color: $body-bg
  color: $text-color
  font-family: $font-primary
```

沒有了Phoenix生成器添加的[Milligram.io](https://milligram.io) CSS框架，網頁的內容不再置中，但可見英文字體變了：

[[Image:210-fig3-first-layout-change]]

接下來可以在`lib/nigiyaka_web/templates/layout/app.html.eex`刪除默認的頁眉並設定網頁標題：

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8"/>
    <meta http-equiv="X-UA-Compatible" content="IE=edge"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <title>Nigiyaka 待辦事清單</title>
    <link rel="stylesheet" href="<%= Routes.static_path(@conn, "/css/app.css") %>"/>
  </head>
  <body>
    <main role="main" class="container">
      <%= render @view_module, @view_template, assigns %>
    </main>
    <script type="text/javascript" src="<%= Routes.static_path(@conn, "/js/app.js") %>"></script>
  </body>
</html>
```

然後在`lib/nigiyaka_web/templates/page/index.html.eex`寫一個示例清單：

```html
<h1>今日待辦事項</h1>
<form class="new-todo-form">
  <input type="text" autocomplete="false" autofocus placeholder="有何貴幹？" />
  <button type="submit">追加</button>
</form>
<ul class="todo-list">
  <li>寫一篇教Phoenix和React的教程</li>
  <li class="is-done">Catch Zapdos</li>
  <li>終於將寫不完的作業寫完</li>
  <li>日本語を話せるようになる</li>
</ul>
```

我們的介面越來越像待辦事清單，但還說不上好看。讓我們在`assets/css/todo_list.sass`添加一點樣式：

```sass
@import "../variables"

ul.todo-list
  color: $text-color

  list-style: none
  margin: 0.5rem 0
  padding-left: 0

  li
    cursor: pointer
    font-size: $todo-font-size
    line-height: 1.2
    padding: $todo-padding-vertical $todo-padding-left
    margin-bottom: 0.25rem
    transition: all .2s

    &:hover
      background-color: $focus-bg

    &.is-done
      text-decoration: line-through
      color: transparentize($text-color, 0.4)
      text-decoration-color: transparentize($text-color, 0.6)

.todo-header
  font-weight: bold
  margin-left: $todo-padding-left
  margin-top: 1rem
  margin-bottom: 1rem
```

現在清單和標題已經很整齊，可是表格仍是默認的樣子。讓我們把表格弄得更漂亮一點：

```sass
.new-todo-form
  display: flex
  transition: all .2s

  > *
    margin: 0

  input[type="text"]
    border: none
    color: $text-color
    flex: 1
    font-size: $todo-font-size
    outline: none
    padding-left: $todo-padding-left
    padding:
      bottom: $todo-padding-vertical
      top: $todo-padding-vertical
    transition: all .2s

    &::placeholder
      color: transparentize($text-color, 0.6)

    &:focus
      background-color: $focus-bg

      + button
        background-color: $focus-bg

        &:hover
          background-color: darken($focus-bg, 3%)

  button[type="submit"]
    background: none
    border: none
    color: $text-color
    cursor: pointer
    font-size: $todo-font-size
    font-weight: bold
    outline: none
    padding: 1rem
    transition: all .2s

    &:hover
      background-color: $focus-bg
```

接下來在把內容安置在頁面中間，在`assets/css/base/layout.sass`添加以下代碼：

```sass
@import "../variables"

.container
  width: 100%
  @media #{$medium-up}
    margin: 0 auto
    max-width: $container-width
```

現在的介面應該像下圖：

[[Image:210-fig4-wireframed-list]]

儲存一下更改：

```
git add .
git commit -m "Mock up interface with HTML"
```

### 迭代1：安裝Babel與React
雖然現在的介面已經設計得差不多了，可是它仍然是靜態的，點擊清單上的事項或傳送表格沒有什麼作用。接下來要安裝React，並建立一個動態的介面。

React所利用JavaScript最新的版本，名叫 [ECMAScript 6](https://babeljs.io/docs/en/learn)（簡稱 ES6），然而這種語言還不是所有瀏覽器都能夠運行的，因此在開發React應用的時候必須採用一種編譯器，Babel。Phoenix所生成`webpack.config.js`已經備有Babel，因此安裝React並不難：

```
cd assets
yarn add react react-dom
yarn add --dev @babel/{polyfill,preset-react,plugin-proposal-class-properties,plugin-proposal-object-rest-spread}
```

現在可以讓Webpack使用`@babel/polyfill`，它的用途是讓我們使`async/await`來處理`Promise`對象。

```javascript
// 15-17行改成以下的樣子：
  entry: ['@babel/polyfill', './js/app.js'],
```

然後更新`assets/.babelrc`並重新啟動Phoenix以載入更新的Webpack設定值：

```javascript
{
  "presets": [
    "@babel/preset-env",
    "@babel/preset-react"
  ],
  "plugins": [
    "@babel/proposal-class-properties",
    "@babel/proposal-object-rest-spread"
  ]
}
```

現在可以摸一摸一個檔案：

```
cd assets/js
touch TodoApp.js
```

`lib/nigiyaka_web/templates/page/index.html.eex`中的靜態HTML可以刪掉，將來只需要給我們的應用程式一個可以掛載（mount）的容器：

```html
<div class="todo-app" id="todo-app"></div>
```

在`assets/js/TodoApp.js`寫一個簡單的組件：

```jsx
import React, { Fragment } from 'react';

export default class TodoApp extends React.Component {
  render() {
    return (
      <React.Fragment>
        <h1 className="todo-header">今日待辦事項</h1>
        <form className="new-todo-form">
          <input type="text" autoComplete="off" placeholder="有何貴幹？" />
          <button type="submit">追加</button>
        </form>
        <ul className="todo-list">
          <li>寫一篇教Phoenix和React的教程</li>
          <li className="is-done">Catch Zapdos</li>
          <li>終於將寫不完的作業寫完</li>
          <li>日本語を話せるようになる</li>
        </ul>
      </React.Fragment>
    )
  }
}
```

然後在`assets/js/app.js`展示該組件：

```jsx
import css from "../css/app.sass";
import React from 'react';
import ReactDOM from 'react-dom';
import TodoApp from './TodoApp';

const todoContainer = document.getElementById('todo-app');
todoContainer && ReactDOM.render(<TodoApp />, todoContainer);
```

React所輸出的HTML跟我們剛寫的靜態HTML大致上是一樣的：

[[Image:210-fig5-react-first-iteration]]

React的新手看見在JS當中肆無忌憚地寫HTML可能頗感詫異（至少我第一次看見不知道應該怎麼處理），但其實這並非HTML，而是React的特徵──JSX。Babel在編譯我們代碼的時候就會把它轉換成JavaScript：

```jsx
// 這樣的JSX：
<li className="is-done">Become a Full Stack Elixir Developer</li>
// 將會轉換成以下代碼：
React.createElement("li", { className: "is-done" }, "Become a Full Stack Elixir Developer")
// 甚至可以在同一個組件當中混用：
<ul className="todo-list">
  <li>寫一篇教Phoenix和React的教程</li>
  <li className="is-done">Catch Zapdos</li>
  <li>終於將寫不完的作業寫完</li>
  <li>日本語を話せるようになる</li>
  {React.createElement("li", { className: "is-done" }, "Become a Full Stack Elixir Developer")}
</ul>
```

然而，新手應該注意到兩個要點。第一，有些參數（`props`）的名稱跟跟HTML有所不同，像是上述`className`與`autoComplete`。第二，一個函數同時只能返回一個對象，因此不能並排兩個一級元素：

```jsx
render() {
  return (
    <h1>我是第一個一級元素</h1> // 這樣根本無法編譯
    <h1>我是第二個一級元素</h1>
  )
}

// 正確寫法：
render() {
  return() {
    <React.Fragment>
      <h1>我是二級元素</h1>
      <h1>我也是二級元素</h1>
    </React.Fragment>
  }
}
```

詳見[官方React入門教程中文版](https://react.docschina.org/tutorial/tutorial.html)。

讀到這裡，大家應該都很辛苦，可以先儲存更改：

```
git add .
git commit -m "Stub TodoApp React component"
```

### 迭代2：將介面分為模塊

接下來讓我們把已經設計好的介面改寫成三個模塊：標題、新增任務的表格以及顯示任務的清單。建立一個新的檔案夾並在裡面建立兩個空檔案：

```bash
mkdir -p assets/js/components
cd !$
touch {TodoList,NewTodoForm}.js
```

在`assets/js/components/TodoList.js`填寫以下代碼：

```jsx
import React from 'react';

const TodoItem = ({ title, done }) => {
  const className = done ? 'is-done' : '';
  return <li className={className}>{title}</li>
}

export default class TodoList extends React.Component {
  render() {
    const items = this.props.items.sort((a, b) => b.id - a.id);
    return <ul className="todo-list">
      {items.map(item => (
        <TodoItem key={`item-${item.id}`} {...item} />
      ))}
    </ul>
  }
}
```

這個檔案裡面有兩個組件（Components），第一個叫`TodoItem`。React的組件不一定要寫成類別，如果那個組件沒有state就可以把它寫成一個簡單的函數：它有一個參數，也就是`props`，而返回值為`JSX.Element`對象或一條字串。進一步了解`props`和`state`的概念請參考[組件&`props`](https://react.docschina.org/docs/components-and-props.html)。

下面的`TodoList`將會顯示所有`TodoItem`項目。因為它是一個類別，每一個`TodoList`元素都是一個對象，因此`render`函數體中若需要使用`props`就要寫成`this.props`。這款軟體裡面，剛新增的項目都會出現在清單最上面；每一個項目將會有唯一的`id`，越加新的項目，`id`就越高，因此在`render`的第一行我們先按照`id`降序排列現存的項目。

第二個組件就是新增任務的表格：

```jsx
import React from 'react';

export default class NewTodoForm extends React.Component {
  render() {
    const { title, onChange } = this.props;
    return (
      <form className="new-todo-form">
        <input
          type="text"
          autoComplete="off"
          name="title"
          placeholder="有何貴幹？"
          onChange={onChange}
          value={title}
        />
        <button type="submit">追加</button>
      </form>
    )
  }
}
```

這個組件是表示用的，它沒有狀態，所有功能都來自於父節點。這邊的注意事項是`onChange`函數，它是`props`之一但可以用來更新父節目的`state`。

最後一個模塊就是它上述兩種模塊綁在一塊兒的`TodoApp`：

```jsx
import React, { Fragment } from 'react';
import TodoList from "./components/TodoList";
import NewTodoForm from "./components/NewTodoForm";

const Header = ({ children }) => <h1 className="todo-header">{children}</h1>;

export default class TodoApp extends React.Component {
  state = {
    items: [
      { id: 1, title: "寫一個靜態的React組件", done: true },
      { id: 2, title: "將應用程式模塊化", done: true },
      { id: 3, title: "將事項寫入資料庫", done: false }
    ],
    title: ''
  }

  onChange = (e) => {
    const { name, value } = e.target;
    this.setState({ [name]: value });
  }

  render() {
    const { items, title } = this.state;

    return (
      <Fragment>
        <Header>今日待辦事項</Header>
        <NewTodoForm onChange={this.onChange} title={title} />
        <TodoList items={items} />
      </Fragment>
    )
  }
}
```

#### 新增項目

現在我們程式所顯示的待辦事項已經都是在`TodoApp`組件裡面儲存的，然而還沒辦法增加新的事項，但我們按“追加”按鈕，什麼都不會發生。下一步可以處理傳送表格，在`TodoApp`類別中添加一些函數並更新`render`：

```jsx
handleFormSubmit = (e) => {
  e.preventDefault();

  const { title } = this.state;
  if (!title) return;
  this.addNewItem(title);
  this.setState({title: ''});
}

addNewItem = (title) => {
  const id = new Date().getTime();
  const items = [...this.state.items, { id, title, done: false }];
  this.setState({ items });
}

render() {
  const { items, title } = this.state;

  return (
    <Fragment>
      <Header>今日待辦事項</Header>
      <NewTodoForm
        handleFormSubmit={this.handleFormSubmit}
        onChange={this.onChange}
        title={title}
      />
      <TodoList items={items} />
    </Fragment>
  )
}
```

當一個用戶傳送表格的時候，事件句柄（event handler）應該先取消默認的動作（`e.preventDefault()`）。如果表格中沒有填寫任何字，那麼也可以不用新增項目。而如果表格裡面有文字，那麼才調用`addNewItem`。由於每一個項目都需要一個唯一的`id`，當我們還沒有使用資料庫的時候可以用當前時間。最後，新增項目以後應該重設表格中的輸入框。

#### 標示事項為已辦

我們已經可以新增待辦事項，可是還沒辦法標示哪些項目已經辦好了。因此接下來可以在`TodoApp`類別中增加`toggleItem`函數並將這個函數加進`<TodoList />`元素的屬性中：

```jsx
toggleItem = (id) => {
  this.setState({
    items: this.state.items.map(item => {
      return (item.id === id) ?
        { ...item, done: !item.done } :
        item;
    })
  });
}

render() {
  const { items, title } = this.state;

  return (
    <Fragment>
      <Header>今日待辦事項</Header>
      <NewTodoForm
        handleFormSubmit={this.handleFormSubmit}
        onChange={this.onChange}
        title={title}
      />
      <TodoList items={items} toggleItem={this.toggleItem} />
    </Fragment>
  )
}
```

`toggleItem`雖然看起來很復雜，但其實它唯一功能是複製`state.items`，而如果它在當中找到所需的`id`就會更新它的`done`值。下一步應該更新`TodoList`與`TodoItem`組件：

```jsx
import React from 'react';

const TodoItem = ({ title, done, handleClick }) => {
  const className = done ? 'is-done' : '';
  return (
    <li className={className} onClick={handleClick}>
      {title}
    </li>
  )
}

export default class TodoList extends React.Component {
  render() {
    const items = this.props.items.sort((a, b) => b.id - a.id);
    return <ul className="todo-list">
      {items.map(item => (
        <TodoItem
          key={`item-${item.id}`}
          handleClick={() => this.props.toggleItem(item.id)}
          {...item}
        />
      ))}
    </ul>
  }
}
```

現在按清單上的一個項目就可以把它標為已辦。這個迭代，我們加了好多功能，建議在此儲存更改：

```
git add .
git commit -m "Modularize TodoApp, add event handlers"
```

可是，我們的更改到現在都湍有儲存在資料庫裡面，如果在瀏覽器按“刷新”紐就會徹底失去了。所以，下一部我們就要用Phoenix寫一個JSON API。

### 迭代3：建立JSON API

Phoenix框架備有功能強大的代碼生成器，它可以幫我們用很短的時間建立一個功能完整的JSON API（或是HTML介面），包括資料庫架構、控制器（controllers）、甚至包括測項。

然而，在我們下手生成這個API之前，我們應該先規劃一下該API的資料架構。其實它會非常簡單，每一項只有一個題（`title`）跟完成時間（`done_at`，這樣我們不僅僅可以知道任務完成與否，還可以知道它是在什麼時候完成的。在控制器與路由器一層，我們會需要三個routes，一個是用來載入所有的待辦事項（`GET index`），另一個是用來新增項目（`POST create`），而第三個便是更新完成與否的狀態（建議稱之為`PATCH toggle`）。

以下指令就可以生成幾乎完整的API：

```
$ mix phx.gen.json Todos Item items title:string done_at:datetime
* creating lib/nigiyaka_web/controllers/item_controller.ex
* creating lib/nigiyaka_web/views/item_view.ex
* creating test/nigiyaka_web/controllers/item_controller_test.exs
* creating lib/nigiyaka_web/views/changeset_view.ex
* creating lib/nigiyaka_web/controllers/fallback_controller.ex
* creating lib/nigiyaka/todos/item.ex
* creating priv/repo/migrations/20181107063324_create_items.exs
* creating lib/nigiyaka/todos/todos.ex
* injecting lib/nigiyaka/todos/todos.ex
* creating test/nigiyaka/todos/todos_test.exs
* injecting test/nigiyaka/todos/todos_test.exs

Add the resource to your :api scope in lib/nigiyaka_web/router.ex:

    resources "/items", ItemController, except: [:new, :edit]


Remember to update your repository by running migrations:

    $ mix ecto.migrate
```

打開它所生成的資料庫變遷檔案（在`priv/repo/migrations`檔案夾）可看出它未來資料庫表的架構：

```elixir
defmodule Nigiyaka.Repo.Migrations.CreateItems do
  use Ecto.Migration

  def change do
    create table(:items) do
      add :title, :string
      add :done_at, :naive_datetime

      timestamps()
    end

  end
end
```

進行資料庫變遷：

```
$ mix ecto.migrate
[debug] QUERY OK source="schema_migrations" db=0.4ms
SELECT s0."version"::bigint FROM "schema_migrations" AS s0 FOR UPDATE []
[info] == Running 20181107063324 Nigiyaka.Repo.Migrations.CreateItems.change/0 forward
[info] create table items
[info] == Migrated 20181107063324 in 0.0s
```

更新Phoenix的路由器（`lib/nigiyaka_web/router.ex`）：

```elixir
defmodule NigiyakaWeb.Router do
  use NigiyakaWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_flash
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  # 添加這一段
  scope "/api", NigiyakaWeb do
    pipe_through :api

    resources "/items", ItemController, except: [:new, :edit]
    patch("/items/:id/toggle", ItemController, :toggle)
  end

  scope "/", NigiyakaWeb do
    pipe_through :browser

    get "/", PageController, :index
  end
end
```

現在我們的應用程式有好幾個可以用的routes，想要檢查的時候可以用`mix phx.routes`：

```
$ mix phx.routes
item_path  GET     /api/items             NigiyakaWeb.ItemController :index
item_path  GET     /api/items/:id         NigiyakaWeb.ItemController :show
item_path  POST    /api/items             NigiyakaWeb.ItemController :create
item_path  PATCH   /api/items/:id         NigiyakaWeb.ItemController :update
           PUT     /api/items/:id         NigiyakaWeb.ItemController :update
item_path  DELETE  /api/items/:id         NigiyakaWeb.ItemController :delete
item_path  PATCH   /api/items/:id/toggle  NigiyakaWeb.ItemController :toggle
page_path  GET     /                      NigiyakaWeb.PageController :index
```

讓我們試試看這個API能不能用（請確認`mix phx.server`是否還在運行中）：

```
$ curl -w '\n' localhost:4000/api/items
{"data":[]}
```

在伺服器的紀錄中可看見剛剛進來的請求：

```
[info] GET /api/items
[debug] Processing with NigiyakaWeb.ItemController.index/2
  Parameters: %{}
  Pipelines: [:api]
[debug] QUERY OK source="items" db=0.8ms
SELECT i0."id", i0."done_at", i0."title", i0."inserted_at", i0."updated_at" FROM "items" AS i0 []
[info] Sent 200 in 1ms
```

目前資料庫裏面沒有待辦事項，讓我們打開`iex -S mix`來試試新增幾個任務：

```elixir
[~/working/nigiyaka] $ iex -S mix
Erlang/OTP 21 [erts-10.1] [source] [64-bit] [smp:4:4] [ds:4:4:10] [async-threads:1] [hipe]

Interactive Elixir (1.7.3) - press Ctrl+C to exit (type h() ENTER for help)
iex(1)> alias Nigiyaka.Todos    
Nigiyaka.Todos
iex(2)> Todos.create_item(%{title: "Catch Zapdos"})
{:error,
 #Ecto.Changeset<
   action: :insert,
   changes: %{title: "Catch Zapdos"},
   errors: [done_at: {"can't be blank", [validation: :required]}],
   data: #Nigiyaka.Todos.Item<>,
   valid?: false
 >}
```

我們無法寫入這樣的項目。原因是`Nigiyaka.Todos.Item`中的`changeset/2`函數中，生成器幫我們把所有的欄標為必填。更改`changeset/2`：

```elixir
@doc false
def changeset(item, attrs) do
  item
  |> cast(attrs, [:title, :done_at])
  |> validate_required([:title]) # 改變這一行
end
```

然後重新嘗試新增待辦事項（記得事先重新打開`iex -S mix`或用`recompile`）：

```elixir
iex(6)> recompile
Compiling 1 file (.ex)
:ok
iex(7)> Todos.create_item(%{title: "Catch Zapdos"})                                      
[debug] QUERY OK db=16.0ms decode=2.0ms queue=1.7ms
INSERT INTO "items" ("title","inserted_at","updated_at") VALUES ($1,$2,$3) RETURNING "id" ["Catch Zapdos", ~N[2018-11-07 07:33:22], ~N[2018-11-07 07:33:22]]
{:ok,
 %Nigiyaka.Todos.Item{
   __meta__: #Ecto.Schema.Metadata<:loaded, "items">,
   done_at: nil,
   id: 4,
   inserted_at: ~N[2018-11-07 07:33:22],
   title: "Catch Zapdos",
   updated_at: ~N[2018-11-07 07:33:22]
 }}
```

看看JSON API的答案：

```
$ curl -w '\n' localhost:4000/api/items
{"data":[{"done_at":null,"id":4,"title":"Catch Zapdos"}]}
```

它基本上沒錯，但是我們的前端並不需要具體的`done_at`時間，有`boolean`是足夠的。在`NigiyakaWeb.ItemView`更改`render("item.json", %{item: item})`這個函數：

```elixir
def render("item.json", %{item: item}) do
  %{id: item.id, title: item.title, done: !!item.done_at}
end
```

看看結果：

```
$ curl -w '\n' localhost:4000/api/items
{"data":[{"done":false,"id":4,"title":"Catch Zapdos"}]}
```

`GET index`的部分已經處理好了。其實Phoenix已經幫我們處理`POST create`的部分，但是用`curl`測試POST請求比較複雜，有興趣的讀者可以自己試試看。 接下來我們來教我們的前端載入資料並建立新的項目！

#### 在`componentDidMount`載入資料

為了傳送請求，建議先安裝一個叫`axios`的函數庫。

```
cd assets
yarn add axios
```

接下來我們需要更新`TodoApp`，讓它初次顯示在熒幕上的時候都下載待辦事項的資料。
相關代碼通常可以放在好幾個地方，這邊建議使用[componentDidMount](https://reactjs.org/docs/react-component.html#componentdidmount)。更新`assets/js/TodoApp.js`：

```jsx
import React, { Fragment } from 'react';
import TodoList from "./components/TodoList";
import NewTodoForm from "./components/NewTodoForm";
import axios from 'axios'; // 添加

const Header = ({ children }) => <h1 className="todo-header">{children}</h1>;

export default class TodoApp extends React.Component {
  state = {
    items: [], // 更新
    title: ''
  }

  // 添加
  componentDidMount() {
    this.fetchTodos();
  }

  fetchTodos = async () => {
    const response = await axios.get('/api/items');
    this.setState({ items: response.data.data });
  }

  // 更新
  addNewItem = async (title) => {
    const response = await axios.post('/api/items', { item: { title } });
    const items = [...this.state.items, response.data.data];
    this.setState({ items });
  }

  // 以下無更改，已省略
  // ...
}
```

現在打開瀏覽器就可以看見從資料庫裡取得的資料：

[[Image:210-fig6-first-gen-legendary-pokemon]]

#### `PATCH toggle`

我們需要寫的最後一個功能是更新項目的`done_at`。Phoenix已經幫我們生成了一個`Nigiyaka.Todos`模塊，我們可以在裏面添加一個`toggle_item/1`函數。首先建議在`test/nigiyaka/todos_test.exs`寫兩個測項：

```elixir
test "toggle_item/1 sets done_at when done_at is nil" do
  item = item_fixture(%{done_at: nil})
  refute item.done_at
  {:ok, toggled} = Todos.toggle_item(item)
  assert toggled.done_at
end

test "toggle_item/1 clears done_at when not nil" do
  item = item_fixture(%{done_at: DateTime.utc_now()})
  assert item.done_at
  {:ok, toggled} = Todos.toggle_item(item)
  refute toggled.done_at
end
```

目前我們的測項有很多都是紅的，因為我們改過`NigiyakaWeb.ItemView`，但它不是重點，所以可以先忽略。`toggle_item/1`的實現如下：

```elixir
@doc """
Toggles a todo item's `done_at`. If the timestamp is nil,
it will be set to current timestamp. If the timestamp is
present, it will be set to nil.

## Examples

    iex> toggle_item(%Item{done_at: nil})
    {:ok, %Item{}}

    iex> toggle_item(%Item{done_at: ~N[2018-11-07 02:18:22]})
    {:ok, %Item{done_at: nil}}

"""
def toggle_item(%Item{} = item) do
  item
  |> Item.changeset(%{done_at: toggle_timestamp(item.done_at)})
  |> Repo.update()
end

defp toggle_timestamp(nil), do: DateTime.utc_now()
defp toggle_timestamp(_), do: nil
```

然後在`lib/nigiyaka_web/controllers/item_controller.ex`加`toggle/2`（基本上可以複製+貼上下面的`delete/2`然後把`Todos.delete_item(item)`改成`Todos.toggle_item(item)`）：

```elixir
def toggle(conn, %{"id" => id}) do
  item = Todos.get_item!(id)

  with {:ok, %Item{}} <- Todos.toggle_item(item) do
    send_resp(conn, :no_content, "")
  end
end
```

前端裡面不需要改太多，只有`TodoApp#toggleItem`：

```javascript
toggleItem = async (id) => {
  await axios.patch(`/api/items/${id}/toggle`);
  this.setState({
    items: this.state.items.map(item => {
      return (item.id === id) ?
        { ...item, done: !item.done } :
        item;
    })
  });
}
```

現在每一個任務完成與否都會寫入資料庫中，刷新後還在呢！謝謝各位讀者的耐心，如果發現有哪一段代碼有錯誤，或者是中文表達方式哪裏有問題請直接用文章下面的表格留下意見回饋。

