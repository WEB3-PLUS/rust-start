### 1. **ایجاد پروژه با Cargo**

برای شروع یک پروژه جدید در Rust، می‌توانید از دستور زیر استفاده کنید:

```bash
cargo new projectName
```

این دستور یک دایرکتوری جدید به نام `projectName` ایجاد می‌کند و در آن یک پروژه جدید با ساختار اولیه و فایل‌های لازم (از جمله `Cargo.toml` و یک فایل `main.rs`) ایجاد می‌کند.

### 2. **ساخت پروژه**

برای ساخت پروژه، از دستور زیر استفاده می‌شود:

```bash
cargo build
```

این دستور کد شما را کامپایل می‌کند و فایل‌های اجرایی را در دایرکتوری `target/debug` ذخیره می‌کند. این دایرکتوری حاوی باینری‌هایی است که برای توسعه و تست استفاده می‌شوند.

### 3. **اجرای پروژه**

برای اجرای برنامه‌ی Rust، می‌توانید از یکی از دو روش زیر استفاده کنید:

- **اجرای مستقیم باینری:**

  ```bash
  ./target/debug/projectName
  ```

  این روش به شما امکان می‌دهد تا باینری ساخته شده را به صورت مستقیم اجرا کنید.

- **استفاده از Cargo:**
  ```bash
  cargo run
  ```
  این دستور به طور خودکار کد شما را کامپایل کرده و سپس آن را اجرا می‌کند. این روش برای توسعه‌دهندگان بسیار راحت‌تر است، زیرا نیازی به یادآوری مسیر باینری نیست.

### 4. **بررسی کد بدون تولید باینری**

اگر می‌خواهید کد خود را بررسی کنید و مطمئن شوید که بدون خطا کامپایل می‌شود، می‌توانید از دستور زیر استفاده کنید:

```bash
cargo check
```

این دستور کد شما را سریعاً بررسی می‌کند، اما باینری تولید نمی‌کند. این ویژگی برای شناسایی سریع خطاها و مشکلات در کد بسیار مفید است.

### 5. **ساخت پروژه برای انتشار**

زمانی که پروژه شما آماده‌ی انتشار است و می‌خواهید آن را با بهینه‌سازی‌های لازم کامپایل کنید، می‌توانید از دستور زیر استفاده کنید:

```bash
cargo build --release
```

این دستور کد شما را با بهینه‌سازی‌های خاصی کامپایل می‌کند و باینری تولید شده را در دایرکتوری `target/release` ذخیره می‌کند. باینری‌های تولید شده در این دایرکتوری معمولاً سریع‌تر و بهینه‌تر هستند و برای استفاده در محیط‌های تولید توصیه می‌شوند.

### 6. **بنچمارک‌گیری**

اگر قصد دارید زمان اجرای کد خود را بنچمارک کنید، مهم است که از باینری موجود در `target/release` استفاده کنید. این باینری بهینه‌سازی شده است و نتایج دقیق‌تری از عملکرد کد شما ارائه می‌دهد.

<hr/>

### کد کامل بازی حدس عدد

```rust
use rand::Rng; // برای تولید اعداد تصادفی
use std::cmp::Ordering; // برای مقایسه مقادیر
use std::io; // برای ورودی و خروجی

fn main() {
    println!("guess the number:");

    // تولید یک عدد تصادفی بین 1 تا 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // چاپ عدد مخفی (برای تست)
    println!("the secret number is: {secret_number}");

    loop {
        println!("please input your guess");

        let mut guess = String::new(); // ایجاد یک رشته خالی برای ورودی کاربر
        io::stdin()
            .read_line(&mut guess) // خواندن ورودی کاربر
            .expect("failed to read line"); // مدیریت خطا

        println!("you guessed: {guess}"); // چاپ حدس کاربر

        // تبدیل ورودی کاربر به عدد
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // اگر تبدیل موفق بود، عدد را ذخیره کن
            Err(_) => continue, // اگر خطا بود، حلقه را ادامه بده
        };

        // مقایسه حدس کاربر با عدد مخفی
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // اگر حدس کمتر بود
            Ordering::Greater => println!("Too big!"), // اگر حدس بیشتر بود
            Ordering::Equal => {
                println!("You win!"); // اگر حدس برابر بود
                return; // پایان بازی
            }
        }
    }
}
```

### 1. **وارد کردن کتابخانه‌ها**

```rust
use rand::Rng; // برای تولید اعداد تصادفی
use std::cmp::Ordering; // برای مقایسه مقادیر
use std::io; // برای ورودی و خروجی
```

- **`use rand::Rng;`**: این خط ماژول `Rng` از کتابخانه `rand` را وارد می‌کند که برای تولید اعداد تصادفی استفاده می‌شود. این ماژول توابعی را برای تولید اعداد تصادفی فراهم می‌کند.
- **`use std::cmp::Ordering;`**: ماژول `Ordering` شامل سه حالت `Less`, `Greater`, و `Equal` است که برای مقایسه مقادیر استفاده می‌شود. این مقادیر به ما اجازه می‌دهند تا بفهمیم یک عدد کمتر، بیشتر یا برابر با عدد دیگر است.
- **`use std::io;`**: ماژول `io` برای کار با ورودی و خروجی در Rust استفاده می‌شود. ما از آن برای خواندن ورودی کاربر استفاده خواهیم کرد.

### 2. **تابع اصلی**

```rust
fn main() {
    println!("guess the number:");
```

- **`fn main()`**: این تابع نقطه شروع هر برنامه Rust است. هر برنامه Rust باید یک تابع `main` داشته باشد.
- **`println!`**: این ماکرو برای چاپ متن به کنسول استفاده می‌شود. در اینجا، به کاربر اعلام می‌شود که باید عددی را حدس بزند.

### 3. **تولید عدد تصادفی**

```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

- **`rand::thread_rng()`**: این تابع یک ژنراتور تصادفی برای ترد فعلی ایجاد می‌کند. این ژنراتور برای تولید اعداد تصادفی استفاده می‌شود.
- **`gen_range(1..=100)`**: این تابع یک عدد تصادفی در بازه 1 تا 100 (شامل هر دو) تولید می‌کند. علامت `..=` نشان‌دهنده این است که انتهای بازه نیز شامل می‌شود.

### 4. **چاپ عدد مخفی (برای تست)**

```rust
println!("the secret number is: {secret_number}");
```

- این خط عدد مخفی را چاپ می‌کند. این کار برای تست مفید است، اما در یک بازی واقعی باید این خط را حذف کرد تا کاربر نتواند عدد مخفی را ببیند.

### 5. **حلقه اصلی بازی**

```rust
loop {
    println!("please input your guess");
```

- **`loop { ... }`**: این یک حلقه بی‌نهایت است که به کاربر اجازه می‌دهد تا حدس‌های خود را وارد کند. حلقه تا زمانی که کاربر برنده نشود یا برنامه به صورت دستی متوقف نشود، ادامه خواهد داشت.

### 6. **خواندن ورودی کاربر**

```rust
let mut guess = String::new(); // ایجاد یک رشته خالی برای ورودی کاربر
io::stdin()
    .read_line(&mut guess) // خواندن ورودی کاربر
    .expect("failed to read line"); // مدیریت خطا
```

- **`let mut guess = String::new();`**: یک رشته خالی به نام `guess` ایجاد می‌شود. `mut` به این معنی است که این متغیر قابل تغییر است.
- **`io::stdin().read_line(&mut guess)`**: این خط ورودی کاربر را از کنسول می‌خواند و آن را در متغیر `guess` ذخیره می‌کند. `&mut guess` به این معناست که ما به تابع اجازه می‌دهیم تا محتوای این متغیر را تغییر دهد.
- **`.expect("failed to read line")`**: این متد در صورت بروز خطا، یک پیام خطا چاپ می‌کند و برنامه را متوقف می‌کند. این کار برای اطمینان از این است که ورودی به درستی خوانده شده است.

### 7. **چاپ حدس کاربر**

```rust
println!("you guessed: {guess}"); // چاپ حدس کاربر
```

- این خط حدس کاربر را چاپ می‌کند. این کار برای تأیید به کاربر است که ورودی او به درستی خوانده شده است.

### 8. **تبدیل ورودی به عدد**

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num, // اگر تبدیل موفق بود، عدد را ذخیره کن
    Err(_) => continue, // اگر خطا بود، حلقه را ادامه بده
};
```

- **`guess.trim()`**: این متد فضای خالی را از ابتدا و انتهای رشته حذف می‌کند. این کار برای جلوگیری از خطا در تبدیل رشته به عدد ضروری است.
- **`.parse()`**: این متد سعی می‌کند رشته را به نوع عددی تبدیل کند. در اینجا ما از `u32` (عدد صحیح غیر منفی) استفاده می‌کنیم.
- **`match`**: این ساختار برای بررسی نتیجه‌ی `parse()` استفاده می‌شود:
  - **`Ok(num)`**: اگر تبدیل موفق باشد، عدد در متغیر `num` ذخیره می‌شود و به متغیر `guess` نسبت داده می‌شود.
  - **`Err(_)`**: اگر تبدیل با خطا مواجه شود (به عنوان مثال، اگر کاربر چیزی غیر از عدد وارد نکند)، حلقه دوباره شروع می‌شود و از کاربر خواسته می‌شود که دوباره حدس بزند.

### 9. **مقایسه حدس با عدد مخفی**

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"), // اگر حدس کمتر بود
    Ordering::Greater => println!("Too big!"), // اگر حدس بیشتر بود
    Ordering::Equal => {
        println!("You win!"); // اگر حدس برابر بود
        return; // پایان بازی
    }
}
```

- **`guess.cmp(&secret_number)`**: این متد مقایسه‌ای بین `guess` و `secret_number` انجام می‌دهد و نتیجه را به یکی از مقادیر `Ordering` (کمتر، بیشتر، برابر) برمی‌گرداند.
- **`match`**: با توجه به نتیجه مقایسه، یکی از سه حالت زیر اجرا می‌شود:
  - **`Ordering::Less`**: اگر حدس کاربر کمتر از عدد مخفی باشد، پیام "Too small!" چاپ می‌شود.
  - **`Ordering::Greater`**: اگر حدس کاربر بیشتر از عدد مخفی باشد، پیام "Too big!" چاپ می‌شود.
  - **`Ordering::Equal`**: اگر حدس برابر با عدد مخفی باشد، پیام "You win!" چاپ می‌شود و با استفاده از `return` از تابع `main` خارج می‌شود، که به معنای پایان بازی است.

### 10. **پایان حلقه و برنامه**

- اگر کاربر حدس صحیح را بزند، برنامه خاتمه می‌یابد. در غیر این صورت، حلقه ادامه می‌یابد و از کاربر خواسته می‌شود که دوباره حدس بزند.

### نکات اضافی

- **مدیریت خطا**: در این برنامه، از `expect` برای مدیریت خطاها استفاده شده است. این روش ساده است، اما در برنامه‌های بزرگ‌تر، ممکن است بخواهید از مدیریت خطای پیچیده‌تری استفاده کنید تا به کاربر اطلاعات بیشتری بدهید.
- **توسعه و بهبود**: این بازی می‌تواند با افزودن ویژگی‌های جدید مانند شمارش تعداد حدس‌ها، ارائه گزینه‌های دوباره بازی، یا ذخیره‌سازی رکوردها بهبود یابد.
- **استفاده از ماژول‌ها**: اگر برنامه بزرگ‌تر شود، می‌توانید از ماژول‌ها برای سازماندهی کد خود استفاده کنید. Rust به شما اجازه می‌دهد تا کد خود را به ماژول‌های مختلف تقسیم کنید که به مدیریت بهتر پروژه کمک می‌کند.

<hr/>
<h2>variable and constant in rust</h2>


### 1. متغیرها و تغییرپذیری

```rust
let mut x: i32 = 5; // متغیر قابل تغییر
println!("x value is : {x}");
x = 6;
println!("x value is : {x}");
```

- **متغیرها**: در Rust، متغیرها به طور پیش‌فرض غیرقابل تغییر (immutable) هستند. این به این معنی است که پس از تعریف، نمی‌توانید مقدار آن را تغییر دهید. برای تعریف یک متغیر قابل تغییر، از کلمه کلیدی `mut` استفاده می‌کنیم.
  
- **تعریف نوع**: در اینجا، `x` به عنوان یک عدد صحیح 32 بیتی (`i32`) تعریف شده است. Rust از نوع داده‌های استاتیک استفاده می‌کند، به این معنی که نوع هر متغیر در زمان کامپایل مشخص می‌شود. این ویژگی به بهبود کارایی و ایمنی برنامه کمک می‌کند.

- **چاپ مقادیر**: از ماکرو `println!` برای چاپ مقادیر استفاده می‌شود. با استفاده از `{x}`، مقدار متغیر `x` در خروجی چاپ می‌شود.

- **تغییر مقدار**: مقدار `x` از 5 به 6 تغییر می‌کند و این تغییر در خروجی نشان داده می‌شود. این قابلیت تغییر مقدار متغیرهای قابل تغییر، یکی از ویژگی‌های مهم Rust است.

### 2. ثابت‌ها

```rust
const ONE_DAY_IN_SEC: u64 = 24 * 60 * 60;
println!("one day in second: {ONE_DAY_IN_SEC}");
```

- **تعریف ثابت**: با استفاده از کلمه کلیدی `const`، می‌توان یک مقدار ثابت تعریف کرد که نمی‌تواند تغییر کند. ثابت‌ها باید در زمان کامپایل مشخص شوند و در کل برنامه قابل دسترسی هستند.

- **نوع داده**: در اینجا، `ONE_DAY_IN_SEC` به عنوان یک عدد صحیح بدون علامت 64 بیتی (`u64`) تعریف شده است. این ثابت تعداد ثانیه‌های یک روز را محاسبه می‌کند (24 ساعت × 60 دقیقه × 60 ثانیه).

- **چاپ مقدار ثابت**: مقدار ثابت با استفاده از `println!` چاپ می‌شود. این به شما امکان می‌دهد تا مقادیر ثابت را به راحتی در برنامه‌های خود استفاده کنید.

### 3. سایه‌زنی (Shadowing)

```rust
let y = 5;
let y = y + 1;
{
    let y = y * 2;
    println!("y value is: {y}");
}
println!("y value is : {y}");
```

- **سایه‌زنی**: در Rust، می‌توانید یک متغیر با همان نام دوباره تعریف کنید. این به شما اجازه می‌دهد تا در یک بلوک خاص، متغیر جدیدی با نام مشابه تعریف کنید که بر روی متغیر قبلی سایه می‌زند.

- **مقداردهی اولیه**: `y` ابتدا به 5 مقداردهی می‌شود. سپس، در خط بعدی، `y` به 6 تغییر می‌کند.

- **بلوک داخلی**: در این بلوک، یک `y` جدید تعریف می‌شود که بر روی `y` قبلی سایه می‌زند. این `y` جدید مقدار 12 را دارد (6 × 2). فقط در این بلوک، این `y` معتبر است و در خط بعدی، `y` اصلی (6) چاپ می‌شود. این ویژگی به شما این امکان را می‌دهد که متغیرها را در محدوده‌های مختلف مدیریت کنید.

### 4. نوع داده‌ها

```rust
let age = "18";
let age: u8 = age.parse().expect("type is not casting to string");
println!("age is : {age}");
```

- **تعریف رشته**: `age` به عنوان یک رشته (string) تعریف شده است که مقدار `"18"` را دارد. در Rust، رشته‌ها به صورت پیش‌فرض از نوع `&str` هستند که یک رشته قابل تغییر نیست.

- **تبدیل نوع**: با استفاده از متد `parse`، رشته به نوع `u8` (عدد صحیح 8 بیتی) تبدیل می‌شود. این متد به صورت عمومی برای تبدیل رشته‌ها به انواع مختلف داده‌ها استفاده می‌شود. اگر تبدیل موفقیت‌آمیز نباشد، از `expect` برای مدیریت خطا استفاده می‌شود. این به شما این امکان را می‌دهد که در صورت بروز خطا، پیام خطای مناسبی دریافت کنید.

- **چاپ مقدار**: مقدار نهایی `age` که اکنون از نوع `u8` است، چاپ می‌شود. این روش به شما کمک می‌کند تا ورودی‌های کاربر را به نوع مناسب تبدیل کنید.

### 5. انواع مرکب

```rust
let _tup: (&str, u8, bool) = (&"mohammadreza", 2, false);
let names = ["mohammadreza", "reza"];
let ages: [u8; 2] = [29, 10];
let role = ["test"; 10];
```

- **تاپل**: 
  - `tup` یک تاپل است که شامل سه نوع مختلف داده است: یک رشته (`&str`)، یک عدد صحیح 8 بیتی (`u8`)، و یک مقدار بولی (`bool`). 
  - تاپل‌ها می‌توانند مقادیر مختلفی از انواع مختلف را در خود نگه دارند. تاپل‌ها به شما این امکان را می‌دهند که چندین مقدار را به عنوان یک واحد مدیریت کنید.

- **آرایه‌ها**:
  - `names` یک آرایه از رشته‌ها است که شامل دو نام است. آرایه‌ها در Rust باید دارای اندازه ثابت باشند.
  - `ages` یک آرایه از نوع `u8` است که شامل دو سن است.
  - `role` یک آرایه است که 10 بار مقدار `"test"` را تکرار می‌کند. این نوع تعریف آرایه به شما این امکان را می‌دهد که یک آرایه با اندازه ثابت و مقادیر تکراری ایجاد کنید.

### 6. ورودی از کاربر و دسترسی به آرایه

```rust
let a = [1, 2, 3, 4, 5];

println!("please enter array index : ");
let mut index = String::new();

io::stdin()
    .read_line(&mut index)
    .expect("failed to read line");
let index: usize = index.trim().parse().expect("index entered is not a number");

let element = a[index];

println!("the index value is {element}");
```

- **تعریف آرایه**: `a` یک آرایه از اعداد صحیح است که شامل 5 عدد (1 تا 5) است. آرایه‌ها در Rust دارای اندازه ثابت هستند و می‌توانند مقادیر مشابه یا متفاوتی را نگه دارند.

- **ورودی از کاربر**:
  - با استفاده از `println!` از کاربر خواسته می‌شود که ایندکس آرایه را وارد کند.
  - یک رشته خالی به نام `index` تعریف می‌شود و با استفاده از `io::stdin().read_line()` ورودی کاربر خوانده می‌شود. این ورودی به `index` اضافه می‌شود.
  - `expect` برای مدیریت خطا در صورت عدم موفقیت در خواندن ورودی استفاده می‌شود. این به شما کمک می‌کند تا در صورت بروز خطا، پیام خطای مناسبی دریافت کنید.

- **تبدیل ورودی**: ورودی کاربر که به صورت رشته است، با استفاده از `trim()` و `parse()` به نوع `usize` تبدیل می‌شود. این نوع برای ایندکس‌ها مناسب است و به شما این امکان را می‌دهد که به عناصر آرایه دسترسی پیدا کنید.

- **دسترسی به عنصر آرایه**: عنصر مربوط به ایندکس وارد شده از آرایه `a` خوانده می‌شود. این بخش از کد می‌تواند منجر به خطا شود اگر کاربر ایندکس نامعتبری وارد کند (مثلاً ایندکس‌های خارج از محدوده آرایه).

- **چاپ مقدار**: مقدار عنصر آرایه با استفاده از ایندکس چاپ می‌شود. این به شما امکان می‌دهد تا ببینید که کدام عنصر از آرایه با ایندکس وارد شده مطابقت دارد.

### نکات اضافی

- **مدیریت خطا**: استفاده از `expect` به شما کمک می‌کند تا در صورت بروز خطا، پیام خطای مناسبی دریافت کنید. این روش به شما امکان می‌دهد تا در زمان توسعه و دیباگ کردن، مشکلات را سریع‌تر شناسایی کنید.

- **نوع داده‌های استاتیک**: Rust به خاطر نوع داده‌های استاتیک و ایمنی حافظه مشهور است. این ویژگی‌ها باعث می‌شوند که بسیاری از خطاهای رایج در زمان کامپایل شناسایی شوند، که این امر به کاهش خطاها در زمان اجرا کمک می‌کند.

- **قابلیت‌های اضافی Rust**: Rust دارای ویژگی‌های دیگری نیز هست که می‌تواند به شما کمک کند، مانند:
  - **مدیریت حافظه بدون جمع‌آوری زباله (Garbage Collection)**: Rust از یک سیستم مالکیت (Ownership) استفاده می‌کند که به شما این امکان را می‌دهد که حافظه را به طور ایمن مدیریت کنید بدون اینکه نیاز به جمع‌آوری زباله داشته باشید.
  - **مدیریت همزمانی**: Rust به شما این امکان را می‌دهد که برنامه‌های همزمان را به راحتی بنویسید و از خطاهای رایج در برنامه‌های همزمان جلوگیری کنید.
  - **ماکروها**: Rust از ماکروها برای تولید کد به صورت خودکار استفاده می‌کند که می‌تواند به کاهش تکرار و ساده‌سازی کد کمک کند.

<hr/>
<h2>functions in rust</h2>


### 1. وارد کردن ماژول `io`

```rust
use std::io;
```

- **وارد کردن ماژول**: این خط ماژول `io` را از کتابخانه استاندارد Rust وارد می‌کند. ماژول `io` شامل توابعی برای کار با ورودی و خروجی (I/O) است، مانند خواندن ورودی از کاربر.

### 2. تعریف تابع `sum`

```rust
fn sum(a: i32, b: i32) -> bool {
    let sum = a + b;
    println!("sum: {sum}");
    return true;
}
```

- **تعریف تابع**: تابع `sum` با دو پارامتر `a` و `b` از نوع `i32` (عدد صحیح 32 بیتی) تعریف شده است.
  
- **محاسبه مجموع**: درون تابع، مقدار مجموع `a` و `b` محاسبه شده و در متغیر `sum` ذخیره می‌شود.

- **چاپ مجموع**: با استفاده از ماکرو `println!`، مقدار مجموع چاپ می‌شود. `{sum}` به مقدار متغیر `sum` اشاره دارد.

- **نوع بازگشتی**: تابع `sum` از نوع `bool` (بولی) است، اما در اینجا همیشه مقدار `true` را باز می‌گرداند. در واقع، در این کد، مقدار بازگشتی چندان کاربردی ندارد.

### 3. تابع `main`

```rust
fn main() {
    println!("Hello, world!");
    let mut a = String::new();
    let mut b = String::new();
    io::stdin().read_line(&mut a).expect("value is not number");
    io::stdin().read_line(&mut b).expect("value is not number");
    let a: i32 = a.trim().parse().expect("error");
    let b: i32 = b.trim().parse().expect("error");
    sum(a, b);
}
```

- **تعریف تابع `main`**: این تابع نقطه شروع اجرای برنامه است.

- **چاپ پیام خوشامدگویی**: با استفاده از `println!`، پیام `"Hello, world!"` چاپ می‌شود.

- **تعریف متغیرها**: دو متغیر `a` و `b` به عنوان رشته‌های خالی (`String::new()`) تعریف شده‌اند. این متغیرها برای ذخیره ورودی کاربر استفاده می‌شوند.

- **خواندن ورودی از کاربر**:
  - با استفاده از `io::stdin().read_line(&mut a)`، برنامه از کاربر می‌خواهد که یک خط ورودی وارد کند و آن را در متغیر `a` ذخیره می‌کند.
  - `expect("value is not number")` به شما این امکان را می‌دهد که در صورت بروز خطا در خواندن ورودی، یک پیام خطا چاپ کنید.

- **تبدیل ورودی به عدد صحیح**:
  - ورودی کاربر که به صورت رشته است، با استفاده از `trim()` (برای حذف فضاهای اضافی) و `parse()` به نوع `i32` تبدیل می‌شود.
  - اگر تبدیل موفقیت‌آمیز نباشد، `expect("error")` پیام خطا چاپ می‌کند.

- **فراخوانی تابع `sum`**: در نهایت، تابع `sum` با مقادیر `a` و `b` به عنوان آرگومان فراخوانی می‌شود.

### نکات اضافی

1. **مدیریت خطا**: استفاده از `expect` در این کد به شما کمک می‌کند تا در صورت بروز خطا، پیام خطای مناسبی دریافت کنید. این روش به شما امکان می‌دهد تا در زمان توسعه و دیباگ کردن، مشکلات را سریع‌تر شناسایی کنید.

2. **نوع بازگشتی تابع**: در این کد، تابع `sum` از نوع `bool` است، اما هیچ دلیلی برای این نوع بازگشتی وجود ندارد، زیرا مقدار بازگشتی هیچ کاربردی ندارد. می‌توانید نوع بازگشتی را به `()` (واحد) تغییر دهید که نشان‌دهنده عدم وجود مقدار بازگشتی است.

3. **بهبود کد**: اگر فقط می‌خواهید مجموع دو عدد را محاسبه کنید، می‌توانید از نوع بازگشتی `()` استفاده کنید و مقدار `true` را از تابع حذف کنید. این کار باعث ساده‌تر شدن کد می‌شود.

<hr/>
<h2>loops and conditions in rust</h2>

### 1. بررسی نمره (Grade)

```rust
fn main() {
    let grade = 1;
    if grade >= 12 && grade <= 20 {
        println!("pass")
    } else if grade >= 7 {
        println!("failed")
    } else {
        println!("ridi");
    }
```

- **تعریف متغیر**:
  - `let grade = 1;` یک متغیر به نام `grade` تعریف می‌کند و به آن مقدار 1 اختصاص می‌دهد. این متغیر نشان‌دهنده نمره یک دانش‌آموز است.

- **شرط‌ها**:
  - **شرط اول**: `if grade >= 12 && grade <= 20`:
    - این شرط بررسی می‌کند که آیا نمره در بازه 12 تا 20 (شامل) قرار دارد. اگر این شرط برقرار باشد، پیام `"pass"` چاپ می‌شود.
    - عملگر `&&` به معنای "و" است و به این معنی است که هر دو شرط باید درست باشند. در اینجا، نمره باید هم بزرگتر یا مساوی 12 و هم کمتر یا مساوی 20 باشد.
  
  - **شرط دوم**: `else if grade >= 7`:
    - اگر نمره در بازه 7 تا 11 باشد (و نه در بازه قبلی)، پیام `"failed"` چاپ می‌شود. این به این معناست که نمره در حد قبولی نیست اما به اندازه‌ای خوب است که به طور کامل مردود نشود.
  
  - **شرط سوم**: `else`:
    - اگر هیچ‌یک از شرایط قبلی برقرار نباشد، یعنی نمره کمتر از 7 باشد، پیام `"ridi"` چاپ می‌شود. این پیام به نظر می‌رسد یک اصطلاح غیررسمی یا طنزآمیز باشد و ممکن است به معنای "خنده‌دار" یا "مسخره" باشد.

### 2. حلقه `loop`

```rust
    //loop
    let mut count = 0;
    'counter: loop {
        if count == 100 {
            println!("number: {count}");
            break 'counter;
        }
        if count % 2 == 0 {
            println!("number: {count}")
        }
        count += 1;
    }
```

- **تعریف متغیر شمارنده**:
  - `let mut count = 0;` یک متغیر به نام `count` تعریف می‌کند و آن را به 0 مقداردهی می‌کند. این متغیر به عنوان شمارنده در حلقه استفاده می‌شود.

- **حلقه بی‌پایان**:
  - `'counter: loop { ... }` یک حلقه بی‌پایان ایجاد می‌کند. نام `'counter` به این حلقه اختصاص داده شده است تا بتوانیم از آن در دستور `break` استفاده کنیم. این نام‌گذاری به ما این امکان را می‌دهد که از چندین حلقه در یک بلوک کد استفاده کنیم و به راحتی از یک حلقه خاص خارج شویم.

- **شرط خروج از حلقه**:
  - `if count == 100`:
    - اگر مقدار `count` برابر با 100 شود، پیام `"number: {count}"` چاپ می‌شود و با استفاده از `break 'counter` از حلقه خارج می‌شویم. در اینجا، `{count}` مقدار شمارنده را در خروجی نمایش می‌دهد.

- **چاپ اعداد زوج**:
  - `if count % 2 == 0`:
    - اگر `count` عددی زوج باشد (یعنی باقی‌مانده تقسیم بر 2 برابر با 0 باشد)، مقدار آن چاپ می‌شود. این به ما امکان می‌دهد تا تمام اعداد زوج بین 0 و 100 را مشاهده کنیم.

- **افزایش شمارنده**:
  - `count += 1;`:
    - در نهایت، مقدار `count` به اندازه 1 افزایش می‌یابد. این خط اطمینان می‌دهد که حلقه به سمت پایان پیش می‌رود.

### 3. حلقه `while`

در کد شما به حلقه `while` اشاره شده، اما در واقع کدی برای `while` وجود ندارد. اما می‌توانیم به صورت کلی بگوییم که در Rust می‌توانیم از حلقه `while` به شکل زیر استفاده کنیم:

```rust
let mut count = 0;
while count < 100 {
    // انجام کارها
    count += 1;
}
```

حلقه `while` به شما این امکان را می‌دهد که تا زمانی که یک شرط خاص برقرار باشد، کد را تکرار کنید. در اینجا، حلقه تا زمانی که `count` کمتر از 100 باشد، ادامه می‌یابد.

### 4. حلقه `for`

```rust
    for element in (1..4).rev() {
        println!("element: {element}");
    }
```

- **حلقه `for`**:
  - این حلقه بر روی یک رنج (range) از اعداد تکرار می‌کند. `(1..4)` یک رنج از 1 تا 3 (شامل 1 و غیرشامل 4) را تولید می‌کند. توجه داشته باشید که در Rust، رنج‌ها شامل عدد ابتدایی و غیرشامل عدد انتهایی هستند.
  
- **معکوس کردن رنج**:
  - `rev()` متدی است که رنج را معکوس می‌کند، بنابراین حلقه از 3 به 1 تکرار می‌شود. در این حالت، `element` به ترتیب 3، 2 و 1 خواهد بود.

- **چاپ مقادیر**:
  - در هر تکرار، مقدار `element` چاپ می‌شود. این به شما امکان می‌دهد تا ببینید که چه مقادیری در هر تکرار از حلقه تولید می‌شوند.

### 5. جمع‌بندی

کد شما شامل چهار بخش اصلی است:
1. **بررسی نمره**: با استفاده از شرط‌ها، نمره یک دانش‌آموز را بررسی می‌کند و پیام مناسب را چاپ می‌کند.
2. **حلقه `loop`**: یک حلقه بی‌پایان که شمارنده را از 0 تا 100 افزایش می‌دهد و اعداد زوج را چاپ می‌کند.
3. **حلقه `while`**: اگرچه در این کد وجود ندارد، اما می‌توان از آن برای تکرار بر اساس یک شرط خاص استفاده کرد.
4. **حلقه `for`**: یک حلقه که بر روی یک رنج معکوس از 3 تا 1 تکرار می‌کند و مقادیر را چاپ می‌کند.

### 6. نکات اضافی

- **مدیریت خطا**: در این کد، هیچ مدیریت خطایی وجود ندارد. در شرایط واقعی، بهتر است برای ورودی‌ها و تبدیل‌ها از مدیریت خطا استفاده کنید تا از بروز مشکلات جلوگیری شود.
  
- **عملگرهای منطقی**: استفاده از عملگر `&&` برای ترکیب شرایط در شرط‌ها به شما این امکان را می‌دهد که چندین شرط را به طور همزمان بررسی کنید.

- **حلقه‌های تودرتو**: اگر بخواهید، می‌توانید از حلقه‌های تودرتو (nested loops) نیز استفاده کنید. این کار به شما این امکان را می‌دهد که ساختارهای پیچیده‌تری از تکرار را پیاده‌سازی کنید.

- **بهینه‌سازی کد**: می‌توانید برخی از بخش‌های کد را بهینه کنید، مانند استفاده از توابع برای جدا کردن منطق و بهبود خوانایی کد.

