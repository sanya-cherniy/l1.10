use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Создаем канал 1 в который будут записываться числа из массива
    let (sender1, mut receiver1) = mpsc::channel(32);
    // Создаем канал 2 в который будут записываться квадраты чисел
    let (sender2, mut receiver2) = mpsc::channel(32);

    // Создаем задачу, которая будет читать данные из первого канала и отправлять во второй канал
    let sender2_clone = sender2.clone();
    let handle1 = task::spawn(async move {
        // ждем сообщения из первого канала
        while let Some(message) = receiver1.recv().await {
            let message = message * message; // вычисляем квадрат полученного числа
            sender2_clone.send(message).await.unwrap(); // отправляем результат во второй канал
        }
        println!("The channel is closed.");
    });
    let handle2 = task::spawn(async move {
        // ждем сообщения из второго канала
        while let Some(message) = receiver2.recv().await {
            println!("Message received: {}", message);
        }
        println!("The channel is closed.");
    });

    // Отправка сообщений в первый канал
    for i in 0..10 {
        sender1.send(i).await.unwrap();
        sleep(Duration::from_millis(500)).await;
    }

    // Закрываем отправитель
    drop(sender1);
    drop(sender2);
    // Ждем завершения задачи
    let _ = handle1.await;
    let _ = handle2.await;
}
