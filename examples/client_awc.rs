
use awc::{
    Client,
    ws::Message,
    ws::Frame,
};
use futures::sink::SinkExt;
use futures::stream::StreamExt;

#[actix_web::main]
async fn main ( ) {
    // connect to ws server
    let client = Client::default ( )
        .ws ( "ws://127.0.0.1:23232/ws" )
        //.ws ( "ws://echo.websocket.org" )
        .connect ( )
        .await
        .unwrap ( )
        ;

    let mut client = client.1;

    ////////////////////////////////////////////////////////////////////////////////
    // test
    let mut input : String = String::new ( );
    let stdin = std::io::stdin ();

    stdin.read_line ( &mut input ).unwrap ( );

    // send to server to check response
    client.send (
        Message::Text (
            input
        )
    ).await
    .unwrap ( );

    while let Some ( response ) = client.next ( ).await {
        println ! ( "Return: {:?}", response );
        if let Ok ( Frame::Ping ( _ ) ) = response {
            client.send (
                Message::Pong ( "".into ( ) )
            ).await
            .unwrap ( );
        } else if let Err ( _ ) = response {
            break;
        }
    }

    &client.close ( )
        .await
        .unwrap ( );
}