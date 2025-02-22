# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [✅] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [✅] Commit: `Create Subscriber model struct.`
    -   [✅] Commit: `Create Notification model struct.`
    -   [✅] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [✅] Commit: `Implement add function in Subscriber repository.`
    -   [✅] Commit: `Implement list_all function in Subscriber repository.`
    -   [✅] Commit: `Implement delete function in Subscriber repository.`
    -   [✅] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [✅] Commit: `Create Notification service struct skeleton.`
    -   [✅] Commit: `Implement subscribe function in Notification service.`
    -   [✅] Commit: `Implement subscribe function in Notification controller.`
    -   [✅] Commit: `Implement unsubscribe function in Notification service.`
    -   [✅] Commit: `Implement unsubscribe function in Notification controller.`
    -   [✅] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [✅] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [✅] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [✅] Commit: `Implement publish function in Program service and Program controller.`
    -   [✅] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [✅] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. Menurut saya, kasus BambangShop belum dibutuhkan interface (atau trait di Rust). Hal ini karena hanya ada satu jenis Subscriber disini. Hasilnya hanya ada satu class yang menjadi Observer. Jika sudah banyak class yang menjadi Observer, maka barulah dibutuhkan sebuah interface. Maka dari itu, saya merasa satu model struct masih cukup untuk kasus BambangShop.

2. Dalam kasus ini, menggunakan Vec atau list tidaklah cukup. Vec dalam Rust memperbolehkan adanya data yang duplikat. Sedangkan DashMap akan memastikan bahwa datanya unik karena tidak memperbolehkan duplikat. Selain itu, DashMap juga dibutuhkan untuk memetakan masing-masing id ke URL dari berbagai subscribernya agar tidak diperlukan 2 Vec berbeda untuk menyimpannya.

3. DashMap merupakan sebuah HashMap yang telah dioptimasi untuk concurrency. Dampaknya DashMap dapat diakses oleh berbagai thread dan dipastikan datanya akan selalu up-to-date. Di sisi lain, singleton pattern merupakan suatu design pattern yang memastikan hanya ada satu instance dari sebuah class. Hal ini menciptakan suatu titik akses yang global bagi semua thread. Hal ini bermanfaat ketika ada class yang me-manage shared data resource dan perlu mempertahankan suatu state tertentu. DashMap menurut saya tetap dibutuhkan karena ada kalanya dibutuhkn struktur data yang cara kerjanya sama seperti HashMap. Maka dari itu, DashMap juga bisa digunakan untuk program yang relatif simpel sehingga tidak perlu mengimplementasi singleton.

#### Reflection Publisher-2
1. Service bertanggung jawab untuk handle kode yang merupakan business logic dari programnya, misalnya mengolah data yang didapatkan dari Repository. Sedangkan Repository merupakan bagian yang berhubungan langsung dengan database dan mengubah isi dari database. Service dan Repository perlu dipisahkan dari Model agar fungsionalitas lebih khusus dan kecil untuk tiap modulnya. Hal ini memenuhi prinsip single responsibility yang memberi banyak dampak positif, seperti memudahkan decoupling, meningkatkan maintainability, memudahkan pembuatan unit test, dan kode lebih bisa di-reuse. 

2. Jika kode yang dibuat hanya menggunakan Model, maka semua macam fungsionalitas (definisi suatu model, business logic, mengakses database, dan mengatur routing) akan digabung menjadi satu function semua. Hal ini akan menciptakan kode yang sangat kompleks dan sulit untuk dibaca maupun di-maintain. Pembuatan unit testing juga akan sangat sulit. Selain itu, akan tercipta coupling yang tinggi sehingga akan menyebabkan banyak perubahan yang harus dilakukan ketika ada satu bagian yang ingin diubah. Hal ini sangatlah tidak direkomendasikan dan bukan best practice dalam membuat codebase.

3. Saya merasa bahwa Postman sangat membantu agar saya mengetahui apakah endpoint yang saya buat telah berfungsi sesuai dengan yang saya harapkan. Meskipun saya bisa menggunakan inspect element, menurut saya penyajian di Postman lebih rapih dan lebih memudahkan saya dalam memvalidasi hasil pekerjaan saya. Selain itu, Postman juga bisa digunakan untuk mencoba mengirimkan data apa saja pada endpoint yang saya buat dan melihat hasilnya. Hal ini sangat bermanfaat untuk memastikan program bekerja dengan baik pada segala kondisi. Saya tertarik untuk explore berbagai tipe data yang dapat dikirim pada body request, params, mencoba berbagai authorization yang disediakan.

#### Reflection Publisher-3
1. BambangShop menggunakan variasi push model. Hal ini dapat dilihat dari create, delete, dan publish product yang akan men-trigger terbuatnya notifikasi untuk subscribers setiap kali functionnya dieksekusi. Object Product disini merupakan publishers dan subscribers merupakan observer. Product disini melakukan pengiriman data sedangkan subscribers hanya menunggu dan menerima data. Maka dari itu, dapat disimpulkan bahwa BambangShop menggunakan push model.

2. Keuntungan yang didapatkan dari pull model adalah setiap subscriber dapat menentukan apakah perubahan data relevan atau tidak bagi mereka. Hal ini membuat notifikasi menjadi lebih sesuai untuk tiap subscribernya. Pull model juga mengurangi coupling antara product dan subscribers. Namun, subscribers jadi harus mengetahui struktur dari product dan kodenya menjadi lebih ribet padahal kebutuhan sistem masih cukup simpel. 

3. Jika tidak dilakukan multi threading pada proses pengiriman notifikasi, maka akan terjadi bottleneck saat jumlah subscriber sangatlah banyak. Setiap subscriber itu harus dibuatkan notifikasi satu-satu secara bergantian sehingga antriannya akan sangat panjang dan terbatas oleh kecepatan komputasi mesin. Maka dari itu, penggunaan multi threading akan lebih efisien.