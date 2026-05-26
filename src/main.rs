use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer};
use local_ip_address::local_ip;
use std::process::Command;

#[get("/apagar")]
async fn apagar_handler() -> HttpResponse {
    println!("> Orden recibida: Apagar pantalla");
    match Command::new("powershell")
        .args([
            "-Command",
            "(Add-Type '[DllImport(\"user32.dll\")] public static extern int SendMessage(int hWnd, int hMsg, int wParam, int lParam);' -Name MonitorControl -PassThru)::SendMessage(-1, 0x0112, 0xF170, 2)",
        ])
        .spawn()
    {
        Ok(_) => {
            println!("> Comando apagar ejecutado exitosamente");
            HttpResponse::Ok().body("Apagando pantalla")
        }
        Err(e) => {
            eprintln!("> ERROR al apagar: {e}");
            HttpResponse::InternalServerError().body(format!("Error: {e}"))
        }
    }
}

#[get("/encender")]
async fn encender_handler() -> HttpResponse {
    println!("> Orden recibida: Encender pantalla");
    match Command::new("powershell")
        .args([
            "-Command",
            "$wsh = New-Object -ComObject Wscript.Shell; $wsh.SendKeys('{SHIFT}')",
        ])
        .spawn()
    {
        Ok(_) => {
            println!("> Comando encender ejecutado exitosamente");
            HttpResponse::Ok().body("Encendiendo pantalla")
        }
        Err(e) => {
            eprintln!("> ERROR al encender: {e}");
            HttpResponse::InternalServerError().body(format!("Error: {e}"))
        }
    }
}

#[get("/status")]
async fn status_handler() -> HttpResponse {
    HttpResponse::Ok().body("OK - Servidor funcionando")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let ip = local_ip().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());

    println!("===========================================");
    println!("  >> matrixJP - Control Remoto v1.0");
    println!("===========================================");
    println!("  [!] Conectate desde tu telefono a:");
    println!("  [!] http://{ip}:{port}");
    println!("===========================================");

    HttpServer::new(|| {
        App::new()
            .service(apagar_handler)
            .service(encender_handler)
            .service(status_handler)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
