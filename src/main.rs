
use actix::prelude::*;
use actix_web::{
    self,
    web,
    HttpServer,
    HttpRequest,
    HttpResponse,
    App,
    Error,
};
use actix_web_actors::ws;
use std::time::{
    Duration,
    Instant,
};

const TIMEOUT : Duration = Duration::from_secs ( 10 );
const HEARTBEAT_INVERVAL: Duration = Duration::from_secs ( 5 );

struct WsActor {
    heartbeat: Instant,
}

impl WsActor {
    fn new ( ) -> Self {
        WsActor {
            heartbeat: Instant::now ( ),
        }
    }

    fn handle_heartbeat ( & self, ctx: &mut <Self as Actor>::Context ) {
        ctx.run_interval ( HEARTBEAT_INVERVAL, |actor, ctx| {
            if Instant::now ( ).duration_since ( actor.heartbeat ) > TIMEOUT {
                ctx.stop ( );
            } else {
                ctx.ping ( b"" );
            }
        } );
    }
}

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;

    // https://github.com/actix/examples/blob/master/websocket/src/main.rs
    fn started(&mut self, ctx: &mut Self::Context) {
        self.handle_heartbeat ( ctx );
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle ( &mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context ) {
        match msg {
            /*
            Ok ( ws::CloseCode ) => {
                ctx.stop ( );
            },
            Ok ( ws::HandshakeError ) => {
                ctx.stop ( );
            },
            Ok ( ws::ProtocolError ) => {
                ctx.stop ( );
            },
            */
            Ok ( ws::Message::Ping ( msg ) ) => {
                self.heartbeat = Instant::now ( );
                ctx.pong ( &msg );
            },
            Ok ( ws::Message::Pong ( _ ) ) => {
                self.heartbeat = Instant::now ( );
            },
            Ok ( ws::Message::Binary ( msg ) ) => {                    
                println ! ( "Msg: {:?}", msg );
                /*
                let mut msg = msg.into_bytes ( );
                msg.reverse ( );
                ctx.text ( String::from_utf8 ( msg ).unwrap ( ) );   // echo?
                */
            },
            Ok ( ws::Message::Text ( msg ) ) => {                    
                println ! ( "Msg: {}", msg );
                let mut msg = msg.into_bytes ( );
                msg.reverse ( );
                ctx.text ( String::from_utf8 ( msg ).unwrap ( ) );   // echo?
            },
            Ok ( ws::Message::Close ( reason ) ) => {                    
                ctx.close ( reason );
                ctx.stop ( );
            },
            _ => ctx.stop ( ),
        }
    }
}

async fn ws_main ( req: HttpRequest, stream: web::Payload ) -> Result<HttpResponse, Error> {
    ws::start ( WsActor::new ( ), &req, stream )
}

async fn hello ( _req: HttpRequest, _stream: web::Payload ) -> String {
    "Hello".to_string()
}

#[actix_web::main]
async fn main() {
    HttpServer::new ( || {
        App::new ( )
            .service ( web::resource ( "/ws" ).route ( web::get ( ).to ( ws_main ) ) )
            .service ( web::resource ( "/" ).route  ( web::get ( ).to ( hello ) ) )
    } )
    .bind ( "127.0.0.1:23232" )
    .unwrap ( )
    .run ( )
    .await
    .unwrap ( );
}
