fn main() {
/*
Rust Programming: The Complete Developer's Guide
Intoduction

 Rust Programming: The Complete Developer's Guide (2:46)

 Exercise: Meet Your Classmates and Instructor

 Course Projects + Code + Slides + Cheatsheet

 Understanding Your Video Player (notes, video speed, subtitles + more)

 Set Your Learning Streak Goal
Rust Fundamentals

 Intro (2:26)

 Data Types (5:01)

 Variables (6:35)

 Functions (8:12)

 println! (3:04)

 Control Flow with If (8:38)

 Repetition (6:30)

 Setup Rust (4:49)

 Comments (3:36)

 Activity: Functions (7:08)

 Numeric Types & Basic Arithmetic (3:36)

 Activity: Basic Math (5:27)

 Let's Have Some Fun (+ Free Resources)
Making Decisions with Rust

 Control Flow: If & Else (2:33)

 Activity: Logic with If & Else (3:59)

 Activity: Logic with If & Else (4:59)

 Match Expression (4:25)

 Demo: Basic Match (3:11)

 Activity: Basic Match 1 (4:03)

 Activity: Basic Match 2 (5:11)

 Unlimited Updates
Repetition

 The Loop Expression (4:48)

 Activity: Loops (4:58)

 The While Loop (3:22)

 Activity: While Loops (4:45)
Working With Data

 Enums (3:11)

 Demo: Enums (3:43)

 Activity: Enums (6:09)

 Structs (2:47)

 Demo: Structs (2:51)

 Activity: Structs (9:27)

 Tuples (3:31)

 Demo: Tuples (5:49)

 Activity: Tuples (5:55)

 Expressions (3:49)

 Demo: Expressions (5:09)

 Activity: Expressions (7:11)

 Course Check-In
Rust's Memory Model

 Intermediate Memory Concepts (3:34)

 Ownership (6:17)

 Demo: Ownership (6:06)

 Activity: Ownership (4:57)
Data Collections

 Implementing Functionality (9:20)

 Activity: Implementing Functionality (16:30)

 The Vector Data Structure (4:53)

 Vector Basics & For Loops (2:28)

 Activity: Vectors & For Loops (6:47)

 About Strings (4:00)

 Demo: Strings (4:28)

 Activity: Strings (7:23)

 Implement a New Life System
Expanding Knowledge

 Deriving Functionality (6:08)

 Type Annotations (4:07)

 Enums Revisited (3:59)

 Demo: Advanced Match (7:57)

 Activity: Advanced match (10:42)

 The Option Type (6:26)

 Demo: Option (4:26)

 Activity: Option (5:01)

 Generating Documentation (2:24)

 Standard Library API docs (3:33)

 Activity: Standard Library API docs (3:46)
Fallible Functions

 The Result Type (4:40)

 Demo: Result (14:16)

 Activity: Result (6:40)

 Activity: Result & The Question Mark Operator (7:51)
Data Collection: HashMap

 The HashMap Data Structure (4:20)

 Working With HashMaps (5:32)

 Activity: HashMap Basics (8:26)
Easier Data Management

 Basic Closures (4:41)

 Map Combinator (5:01)

 Activity: Map Combinator (8:07)

 Option Combinator Pattern (7:49)

 Activity: Option Combinators (4:23)

 Using Iterators (9:12)

 Activity: Using Iterators (5:14)

 Ranges (1:58)

 If..let..else (2:58)

 while..let (2:06)
Managing Code

 Inline Modules (4:02)

 Activity: Inline Modules (10:17)

 Testing (6:52)

 Activity: Testing (7:42)

 External Crates (6:28)

 Activity: Adding an External Crate (4:44)

 External Modules (10:27)

 Activity: External Modules (9:54)
Milestone Project: Billing Application

 Gathering User Input (14:21)

 Activity: Gathering User Input (22:47)

 Mini Project: Introduction (2:44)

 Retrieve User Input (3:16)

 Creating The Main Menu Loop (7:14)

 Required Data Structures (4:10)

 Implementation: Adding & Viewing Bills (13:00)

 Implementation: Removing Bills (6:50)

 Implementation: Editing Bills (9:15)
Shared Functionality

 Traits (4:54)

 Demo: Traits (4:58)

 Activity: Traits (5:58)

 Implementing The "Default" Trait (2:28)

 Generics & Functions (13:54)

 Demo: Generics & Functions (6:22)

 Activity: Generics & Functions (5:52)

 Generic Structures (10:06)

 Generic Structures & impl Blocks (7:58)

 Demo: Generics & Structures (6:38)

 Activity: Generics & Structures (10:18)

 Advanced Memory Concepts (8:50)

 Trait Objects (11:35)

 Demo: Trait Objects (7:26)

 Activity: Trait Objects (8:35)
Lifetimes

 Ownership & Lifetimes (9:03)

 Demo: Lifetimes (8:32)

 Activity: Lifetimes & Structures (10:49)

 Activity: Lifetimes & Functions (4:47)
Improving Program Reliability

 Custom Error Types (8:51)

 Demo: Custom Error Types (8:53)

 Activity: Creating a Custom Error (9:01)

 const (2:20)

 New Type Pattern (5:07)

 Activity: Utilizing The New Type Pattern (8:08)

 TypeState Pattern (4:13)

 Demo: TypeState Pattern (11:48)

 Activity: TypeState Pattern (7:41)

 Demo: Match Guards & Binding (6:22)

 Activity: Match Guards & Binding (8:45)

 Arrays & Slices (7:48)

 Slice Patterns (5:53)

 Activity: Slices (5:41)

 Type Aliases (5:22)

 Exercise: Imposter Syndrome (2:55)
Type Conversions

 From/Into (8:17)

 TryFrom/TryInto (3:56)

 Demo: From/Into (7:58)

 Activity: TryFrom/TryInto (10:15)

 Numeric Limits & Numeric Type Casting (7:59)
Parallel Execution

 Passing Closures to Functions (10:53)

 Threads (6:44)

 Demo: Threads (7:39)

 Activity: Threads (4:29)

 Channels (8:13)

 Demo: Channels (5:04)

 Demo: Bidirectional Threaded Communication (7:28)

 Activity: Channels (9:23)
Shared Ownership

 Smart Pointers (4:07)

 Interior Mutability: Cell & RefCell (8:06)

 Demo: Smart Pointers & RefCell (6:00)

 Activity: Smart Pointers & RefCell (9:14)

 Arc/Mutex (8:38)

 Threading: Deadlocks (6:27)

 Demo: Arc/Mutex (4:15)

 Activity: Arc/Mutex (5:13)
Standard Library Tour

 Enum Equality & Ordering (3:42)

 Struct Equality & Ordering (4:23)

 Operator Overloading (7:24)

 Iterators: Implementing Iterator for a Struct (3:31)

 Implement IntoIterator (7:58)

 Demo: Implementing IntoIterator (9:23)

 Activity: Implementing Iterator (5:45)

 Iterators: Custom Iteration Logic (8:07)

 Helpful Macros (7:48)

 Managing Integer Overflow (5:39)
Other Language Features

 Turbofish (2:29)

 Loop Labels (2:44)

 Loop Expressions (2:34)

 Struct Update Syntax (2:47)

 Escape Sequences & Raw Strings (4:17)
Development Experience

 rust-analyzer (4:18)

 clippy (2:31)

 error-lens (1:27)
Crate Roundup

 dotenvy (2:32)

 serde (3:56)

 rand (4:20)

 cached (4:16)

 regex (5:17)

 chrono (7:31)

 strum (4:24)

 derive_more (6:32)

 rayon (4:00)

 tracing (6:29)

 color-eyre (4:15)
Resources

 Helpful Links for Your Rust Career
Final Project: Clip Stash

 Async Primer (7:51)

 Introduction To The Project (1:03)

 Architecture (3:38)

 Walkthrough & Domain Structure Modules (7:25)

 Domain Errors (6:02)

 Domain Implementation: Content & Hits (6:36)

 Domain Implementation: Password (6:27)

 Domain Implementation: Shortcode (7:32)

 Domain Implementation: Title (2:40)

 Domain Implementation: Time (6:48)

 Domain Implementation: Expire & Posted (5:03)

 Domain Implementation: Dbld & Clipld (6:00)

 Recap & Error Correction (5:18)

 Database Type Aliases (7:25)

 Database Wrapper (4:12)

 Database Model (11:56)

 SQL Primer (4:25)

 sqlx cli (1:42)

 Database Query: Get Clip (6:55)

 Database Query: New Clip (7:06)

 Database Query: Update Clip (3:51)

 Recap & Next Steps (4:18)

 Service Layer: Errors (7:34)

 Service Layer: Get Clip (11:22)

 Service Layer: Add & Update Clip (7:21)

 Templates (2:00)

 Page Contexts (6:15)

 Template Renderer (14:30)

 Rocket Framework (4:28)

 Initial Web Setup (3:46)

 Web Forms (9:19)

 Homepage, Catchers, and Router (5:45)

 Rocket Configuration (3:34)

 Rocket Configuration: Troubleshooting

 Running the Server (9:34)

 Retrieving a Clip (11:05)

 Saving a Clip (13:14)

 Password Protected Clips (7:12)

 Raw Clips (5:31)

 Hit Counter (1:39)

 Hit Counter - Service & Data (11:38)

 Hit Counter Implementation part 1 (12:27)

 Hit Counter Implementation part 2 (5:58)

 Database Migration (1:11)

 API: Keys & Error Handling (9:19)

 API: Service & Queries (7:10)

 API: Request Guard (6:43)

 API: Routing (12:06)

 API Client: Cargo.toml & CLI Options (8:38)

 API Client: Get Clip & New Clip (7:42)

 API Client: Update Clip & Making Requests (6:07)

 Maintenance Tasks (7:06)

 Testing: Database (9:14)

 Testing: HTTP Routes (10:00)
Declarative Macros

 Overview (6:54)

 Detail (15:39)

 Demo: impl Blocks (6:38)

 Activity: Control Flow (2:51)

 Activity: impl Blocks (2:34)

 Repetitions (8:46)

 Demo: Repetitions (3:14)

 Activity: HashMap (5:22)

 Demo: Syntax Extension (8:52)

 Activity: Syntax Extension (4:11)

 Activity: Generating Tests (6:31)

 Activity: Function Tracer (4:47)

 Demo: Checked Config (13:18)

 Demo: Recursive tt Muncher (6:40)
Where To Go From Here?

 Thank You! (1:17)

 */












}