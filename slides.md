---
marp: true
paginate: true
theme: default
style: |
  @import url('https://fonts.googleapis.com/css2?family=Playfair+Display:wght@700;900&family=Source+Sans+3:wght@400;600&family=Fira+Code:wght@400&display=swap');
  :root {
    --color-background: #f5f0e8;
    --color-foreground: #2b2b2b;
    --color-highlight: #b5000f;
    --color-accent: #0550ae;
  }
  section {
    background: radial-gradient(ellipse at center, #f5f0e8 40%, #e0d6c8 85%, #c9bda8 100%);
    color: var(--color-foreground);
    font-family: 'Source Sans 3', 'Helvetica Neue', sans-serif;
    font-size: 28px;
    border-bottom: 6px solid var(--color-highlight);
    padding: 50px 60px;
  }
  h1, h2, h3 {
    font-family: 'Playfair Display', Georgia, serif;
    color: var(--color-highlight);
    letter-spacing: -0.02em;
  }
  h2 {
    font-size: 1.8em;
    border-bottom: 2px solid #d4c9b8;
    padding-bottom: 0.2em;
    margin-bottom: 0.6em;
  }
  h3 {
    color: var(--color-foreground);
    font-size: 1.2em;
  }
  a {
    color: var(--color-accent);
    text-decoration-color: #b8cceb;
    text-underline-offset: 3px;
  }
  strong {
    color: var(--color-highlight);
  }
  li {
    margin-bottom: 0.3em;
  }
  code {
    font-family: 'Fira Code', monospace;
    background-color: #e8e0d4;
    color: #2b2b2b;
    padding: 0.1em 0.4em;
    border-radius: 4px;
    font-size: 0.9em;
  }
  section::after {
    color: #b5a898;
    font-family: 'Source Sans 3', sans-serif;
    font-size: 0.6em;
  }
  section.lead {
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
    border-bottom: none;
  }
  section.lead h1 {
    font-size: 2.8em;
    line-height: 1.2;
    margin-bottom: 0.1em;
  }
  section.lead p {
    font-size: 1.2em;
    color: #6b6159;
  }
---

<!-- _class: lead -->

# SQLite: More Powerful Than You Think

Mikaël Francoeur

---

## SQLite

- Embedded database
- A single file, or in-memory
- Library ≈650KB
- Since 2000
- Public domain
- 3 developers
- 1 trillion databases worldwide: browsers, phones, planes, cars, TVs, cameras…

---

<!-- _class: lead -->

## Demo: SQLite

---

## Application-Defined SQL Functions

- Simple functions
- Window functions
- Aggregate functions
- Table-valued functions

---

## Vector Extensions

Local Vector Search

- **sqlite-vec**: 
  - in alpha
  - sponsored by Mozilla, Turso, and others
  - Last release in 2024
- **sqlite-vector**: 
  - led by SQLite AI, highly optimized
  - Last release yesterday

---

## Session Extension

- Open a session
- Modify the database
- Create a changeset file
- Apply the generated file to another database to synchronize it

- Can be reversed (A→A' but also A'→A)
- Changesets can be merged
- Even supports "rebase"

---

## sqlite-http

- by Alex Garcia
- pre-v1
- no observability
- good for rapid prototyping (LLM)

---

## zipfile

Read/modify files inside an archive

Write the rows of a query result into an archive, one file per row

### Use cases:
- Find a class/string in a JAR or docx
- Modify data in zip files in a one-time script
- Transform your query results into an archive

---

## Other Capabilities/Extensions

**Built into SQLite:**
- [SQLite Archive](https://sqlite.org/sqlar.html)
- [Spell Checking](https://sqlite.org/spellfix1.html)

**Extensions:**
- [sqlelf](https://github.com/fzakaria/sqlelf)
- [sqlite-js](https://github.com/sqliteai/sqlite-js)
- [litesql/kafka](https://github.com/litesql/kafka)
- [litesql/nats](https://github.com/litesql/nats)

**Replication Tools**

- [libSQL](https://github.com/tursodatabase/libsql)
- [Litestream](https://litestream.io/)
- [sqlite-sync](https://github.com/sqliteai/sqlite-sync)
- [sqlsync](https://github.com/orbitinghail/sqlsync)
- [sqlite-rsync](https://sqlite.org/rsync.html)

---

## Turso

![bg right:35% vertical 50%](img/turso-logo.png)
![bg 80%](img/stars.png)
![bg 80%](img/contributors.png)

- SQLite-compatible
- Async-first
- Encryption, Vector Search
- Multi-Writer
- MIT license
- 17K GH stars
- 200 contributors

---

# Comment, Like and Subscribe

![bg right:35% vertical 50%](img/confoo-sqlite-more-powerful-than-you-think-2026.png)

---

# Link to the code examples

