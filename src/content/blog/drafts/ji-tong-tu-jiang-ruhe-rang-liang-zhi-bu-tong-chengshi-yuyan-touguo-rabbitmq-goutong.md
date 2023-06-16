---
title: 雞同兔講：如何讓兩隻不同程式語言透過 RabbitMQ 溝通
date: 2021-02-20
slug: ji-tong-tu-jiang-ruhe-rang-liang-zhi-bu-tong-chengshi-yuyan-touguo-rabbitmq-goutong
lang: zh-Hant
---

## 摘要

本文描寫了如何透過 RabbitMQ 使 Node.js 與 Elixir 即時溝通，並提供一個模擬專案來展現實際解決方案。

<a href="https://github.com/moroz/rabbitmq_demo" target="_blank" rel="noopener noreferer">Show me the code! / 我要看程式碼！</a>

## 問題介紹

近期有一個專案需要為一個現有的系統新增功能。原始的專案以 Node.js 為後端伺服器。該專案為公司內用系統，規模不大，然而結構卻錯綜複雜。小編認為 Node.js 生態環境最嚴重的問題一向就是沒有可行的 ORM。只要不需要碰到資料庫，Node 就可以寫得很愉快，可惜大部分應用程式都需要資料庫。現有的 ORM 各有優缺點，尤其是文檔沒有完全覆蓋全部的功能與設定，但目前為止最好的應該就是 TypeORM。

TypeORM 雖然在簡單查詢方面尚可，但測試方面表現不佳，無法以資料庫交易（transaction）隔斷每一項測試的資料庫連接，使資料庫互動的測試極為艱難。
這個難點唯一解決方案是 [typeorm-test-transactions](https://www.npmjs.com/package/typeorm-test-transactions) 這個包。
可見該包的作者很努力，但大概是因為 TypeORM 連接池的設計，它只支援很基本的查詢（使用 `Repository` 或 `BaseEntity` 的方法），
而一旦出現比較復雜的查詢（使用 `getConnection()`），就會取得另外一個資料庫連接，使交易無效。

同時，小編最擅長的一門程式語言 Elixir 在單元測試方面的表現非常好：每一個用 `mix phx.new` 建立的 Phoenix 專案都預備 ExUnit 測試框架及支援獨立測試的資料庫連接框架 Ecto。
Elixir 的測項文法簡潔，執行速度非常快。因此，最終的開發策略便是在原始的資料庫結構上額外加上一款用 Phoenix 框架開發的應用程式，與原有的系統共用資料庫。 大部分的時候，共用的資料庫已經足以讓兩款應用程式協力合作，唯有少數的時候需要互相通知。

這個問題最單純解決方案便是在微型服務之間傳送 HTTP 請求。這個方法本身雖簡單，可是無法保證百分之百的準確性。假設在收款當下接受訊息的應用程式當機，無法處理訊息，那麼傳送訊息的那一款應用程式應不應該重試？若重試，如何保證同一個請求不會引起多餘的效果？即便兩款服務都在同一臺機器運作，它們之間的直接連接可能發生各種異常，如果使用 HTTP 請求的單純方案，開發者就要想辦法解決這些問題，保證資料準確性。

因此在使用微型服務的時候，更可靠的作法是使用一種訊息隊列（Message Queue）。業界常用的訊息隊列軟體有 RabbitMQ 與 Apache Kafka。Kafka 品質很好，但太低層，使用上不那麼簡單，一般小型專案不用不到那麼復雜的工具。RabbitMQ 比較小，它使用的協議叫做 AMQP（Advanced Message Queue Protocol，高級消息隊列協議）。

RabbitMQ 與 Kafka 都接受任何格式的二進訊息，加密與解密是工程師的責任。我們的訊息可以用 JSON，但在本文的模擬專案用的資料格式為 Apache Avro。Avro 為一種壓縮的二進格式，主要好處是加密與解密速度快、加密後的資料量很小、還有強型別，加密時將會保證資料格式。

## 範例專案

倘若我們有兩個賣東西的系統，第一個系統為 Node.js 應用程式，第二個為 Phoenix 應用程式。兩個系統都可以下訂單，但是用同一個金流服務，金流公司款要求我們的帳號只能設定一個 webhook 接口。 這代表不論訂單由哪一個系統建立，收款通知只能寄到其中一個處理收款的系統。因此，收款的邏輯保留在第一個系統內。當金流服務傳來收款通知時，將判斷收款的訂單為哪款應用程式所建立，若為 Node 系統所建立，則留給 Node 處理，而若為 Elixir 系統所建立，則另外通知 Elixir。

小編為展現兩個服務之間的溝通，開發了一個模擬專案，該專案的源代碼可至小編的 <a href="https://github.com/moroz/rabbitmq_demo" target="_blank" rel="noopener noreferer">GitHub</a> 取得，該專案以下簡稱 Project Usagi（從日文 うさぎ _usagi_，「兔子」的意思）。

Usagi 由兩款小型應用程式組成（可謂微型服務），分別命名為 Producer （訊息生成端）與 Consumer （訊息接受端）。Producer 與 Consumer 將連接至同一臺 RabbitMQ 伺服器。

讀者若想嘗試

Producer 為一款 Node API，提供一個簡單的接口，可以接受 JSON 格式的付款通知（模擬收款服務）並即時傳送付款通知到 RabbitMQ Elixir。

```bash
curl --location --request POST 'http://localhost:3000/api/payments/fulfill' \
--header 'Content-Type: application/json' \
--data-raw '{"order_id": 42, "fulfilled_at": 1613916290, "payment_method": "WECHAT_PAY"}'
```
