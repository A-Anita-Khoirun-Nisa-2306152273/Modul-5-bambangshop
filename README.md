# Modul 5
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
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [x] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [x] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [x] Commit: `Implement publish function in Program service and Program controller.`
    -   [x] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [x] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. Penggunaan satu model struct sudah mencukupi karena seluruh subscriber memiliki struktur data dan fungsi yang sama, yaitu menerima notifikasi. Penerapan trait atau interface biasanya diperlukan ketika terdapat variasi perilaku (polimorfisme) yang harus mengikuti kontrak tertentu. Dalam kasus ini, kebutuhan tersebut belum ada, sehingga penggunaan trait justru akan menambah kompleksitas tanpa memberikan manfaat yang signifikan.

2. Penggunaan `DashMap` lebih tepat dibandingkan `Vec` karena data subscriber memiliki atribut unik seperti id atau url yang berperan sebagai key. Jika menggunakan `Vec`, setiap operasi pencarian, penghapusan, atau pembaruan harus dilakukan dengan iterasi satu per satu (linear search) yang memiliki kompleksitas O(n), sehingga kurang efisien ketika data semakin banyak. Sebaliknya, `DashMap` memungkinkan akses langsung berdasarkan key dengan kompleksitas rata-rata O(1), sehingga lebih cepat dan efisien.

3. `DashMap` dan pola Singleton bukanlah pilihan yang saling menggantikan, melainkan saling melengkapi. `lazy_static` digunakan untuk memastikan hanya ada satu instance data global (Singleton), sementara `DashMap` memastikan data tersebut aman digunakan dalam lingkungan multi-threading (thread-safe). Dalam aplikasi backend yang menangani banyak request secara bersamaan, penggunaan struktur data yang aman dari data race sangat penting. Jika hanya menggunakan `HashMap` biasa, tetap diperlukan mekanisme tambahan seperti `Mutex` atau `RwLock`, sehingga `DashMap` menjadi solusi yang lebih praktis dan efisien.


#### Reflection Publisher-2
1. Pemisahan antara Model, Service, dan Repository merupakan penerapan prinsip Single Responsibility Principle (SRP), di mana setiap komponen memiliki tugas yang jelas. Model hanya berisi struktur data, Repository menangani operasi penyimpanan (CRUD), dan Service mengelola logika bisnis. Dengan pemisahan ini, kode menjadi lebih terstruktur, mudah diuji, dan fleksibel terhadap perubahan, misalnya saat mengganti database tanpa harus mengubah logika utama aplikasi.

2. Jika semua tanggung jawab digabung dalam Model, maka kode akan menjadi kompleks dan sulit dikelola. Model bisa berubah menjadi “God Object” yang menangani terlalu banyak hal sekaligus, seperti mengelola data, mengatur subscriber, hingga mengirim notifikasi. Hal ini membuat perubahan kecil pada satu bagian dapat berdampak ke banyak bagian lain, sehingga meningkatkan risiko error dan menyulitkan pengembangan serta kolaborasi tim.

3. Penggunaan Postman sangat membantu dalam menguji dan memvalidasi API backend karena memungkinkan kita mengirim dan mengecek request HTTP secara langsung tanpa bergantung pada frontend. Fitur seperti Collections dapat digunakan sebagai dokumentasi API yang bisa dibagikan ke tim, sehingga semua anggota memiliki referensi yang sama. Selain itu, Environment Variables memudahkan pengujian di berbagai lingkungan, dan fitur automated testing dapat mempercepat proses pengecekan kualitas aplikasi.


#### Reflection Publisher-3
1. Pada tutorial ini digunakan model **Push**, di mana publisher secara aktif mengirim data ke subscriber. Hal ini terlihat saat `NotificationService::notify` membuat payload lalu langsung mengirimkannya ke semua subscriber melalui HTTP request. Subscriber bersifat pasif karena hanya menerima data saat terjadi event seperti create, publish, atau delete.

2. Jika menggunakan model **Pull**, subscriber bisa mengambil data sesuai kebutuhan sehingga tidak kewalahan saat terjadi banyak event. Namun, kekurangannya adalah kurang efisien untuk notifikasi real-time karena subscriber harus terus melakukan polling ke server, atau publisher harus menyimpan data lebih lama sampai semua subscriber mengambilnya.

3. Tanpa multi-threading (`thread::spawn`), proses notifikasi akan berjalan secara sinkron dan menjadi lambat. Karena pengiriman notifikasi berupa HTTP request membutuhkan waktu, thread utama akan tertahan. Akibatnya, respons ke pengguna menjadi lama karena harus menunggu semua notifikasi selesai dikirim satu per satu.

