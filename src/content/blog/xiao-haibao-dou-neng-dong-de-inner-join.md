---
title: 小海豹都能懂的 inner join
date: 2020-09-25
slug: xiao-haibao-dou-neng-dong-de-inner-join
tags:
  - Inner Join
  - SQL
  - 中文
---

假設我們要管理寵物資料庫。首先我們要確保電腦上有裝 PostgreSQL:

```bash
$ psql --version
psql (PostgreSQL) 12.4 (Ubuntu 12.4-0ubuntu0.20.04.1)
```

來建立新的資料庫並連接到新建立的資料庫：

```bash
createdb pets
psql pets
```

建立 `humans` 的資料表：

```sql
create type gender as enum('male', 'female', 'non_binary');

create table humans (
  id serial primary key, -- 主鍵
  name text not null,    -- 姓名
  gender gender not null
);
```

建立 `pets` 的資料表：

```sql
create table pets (
  id serial primary key, -- 主鍵
  name text not null,    -- 名字
  species text not null, -- 品種
  owner_id int not null, -- 一隻寵物 belongs to a human
  constraint fk_owner
    foreign key (owner_id)
      references humans (id)
);

create index pets_owner_id_idx on pets (owner_id);
```

新增幾個人：

```sql
insert into humans (name, gender)
values
('John Doe', 'male'),
('Jane Smith', 'female'),
('Marion Green', 'non_binary');
```

我們可以檢查 `humans` 資料表中的資料：

```sql
select * from humans;
```

然後新增一些寵物：

```sql
insert into pets (name, species, owner_id)
values
('Rex', 'Canis lupus familiaris', 1),
('Caesar', 'Canis lupus familiaris', 1),
('Simba', 'Panthera leo', 2);
```

要是我們知道 John Doe 的 ID，我們很容易查到他的寵物有哪些：

```sql
select * from pets where owner_id = 1;
```

然而，我們可能不知道 Jane Smith 的 ID，反而我們知道她的名字。在這種情況下，我們就是要用 INNER JOIN：

```sql
select * from pets
inner join humans on humans.id = pets.owner_id
where humans.name = 'Jane Smith';
```

其實呢，我們並沒有必要寫得這麼冗長，因為 JOIN 預設就是 INNER JOIN:

```sql
select * from pets
join humans on humans.id = pets.owner_id
where humans.name = 'Jane Smith';
```

另外，我們可以為資料表取比較簡短的名稱，比如 `humans` 可以縮寫成 `h`，而 `pets` 可寫為 `p`：

```sql
select * from pets AS p -- AS 可以省略
join humans AS h on h.id = p.owner_id
where h.name = 'Jane Smith';
```
