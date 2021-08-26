use std::rc::Rc;

use futures::{future::LocalBoxFuture, Future};
use matchit::{InsertError, Match, Node, Params};

use crate::{env::Env, http::Method, request::Request, response::Response, Result};

type HandlerFn = fn(Request, Env, Params) -> Result<Response>;
type AsyncHandlerFn<'a> = Rc<dyn Fn(Request, Env, Params) -> LocalBoxFuture<'a, Result<Response>>>;
type RouterResult<T = ()> = std::result::Result<T, InsertError>;

pub enum Handler<'a> {
    Async(AsyncHandlerFn<'a>),
    Sync(HandlerFn),
}

impl Clone for Handler<'_> {
    fn clone(&self) -> Self {
        match self {
            Self::Async(rc) => Self::Async(rc.clone()),
            Self::Sync(func) => Self::Sync(*func),
        }
    }
}

pub type HandlerSet<'a> = [Option<Handler<'a>>; 9];

pub struct Router<'a> {
    handlers: Node<HandlerSet<'a>>,
}

impl<'a> Router<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&mut self, pattern: &str, func: HandlerFn) -> RouterResult {
        self.add_handler(pattern, Handler::Sync(func), vec![Method::Get])
    }

    pub fn post(&mut self, pattern: &str, func: HandlerFn) -> RouterResult {
        self.add_handler(pattern, Handler::Sync(func), vec![Method::Post])
    }

    pub fn on(&mut self, pattern: &str, func: HandlerFn) -> RouterResult {
        self.add_handler(pattern, Handler::Sync(func), Method::all())
    }

    pub fn get_async<T>(
        &mut self,
        pattern: &str,
        func: fn(Request, Env, Params) -> T,
    ) -> RouterResult
    where
        T: Future<Output = Result<Response>> + 'static,
    {
        self.add_handler(
            pattern,
            Handler::Async(Rc::new(move |req, env, par| Box::pin(func(req, env, par)))),
            vec![Method::Get],
        )
    }

    pub fn post_async<T>(
        &mut self,
        pattern: &str,
        func: fn(Request, Env, Params) -> T,
    ) -> RouterResult
    where
        T: Future<Output = Result<Response>> + 'static,
    {
        self.add_handler(
            pattern,
            Handler::Async(Rc::new(move |req, env, par| Box::pin(func(req, env, par)))),
            vec![Method::Post],
        )
    }

    pub fn on_async<T>(
        &mut self,
        pattern: &str,
        func: fn(Request, Env, Params) -> T,
    ) -> RouterResult
    where
        T: Future<Output = Result<Response>> + 'a,
    {
        self.add_handler(
            pattern,
            Handler::Async(Rc::new(move |req, env, par| Box::pin(func(req, env, par)))),
            Method::all(),
        )
    }

    fn add_handler(
        &mut self,
        pattern: &str,
        func: Handler<'a>,
        methods: Vec<Method>,
    ) -> RouterResult {
        // Did some testing and it appears as though a pattern can always match itself
        // i.e. the path "/user/:id" will always match the pattern "/user/:id"
        if let Ok(Match {
            value: handler_set,
            params: _,
        }) = self.handlers.at_mut(pattern)
        {
            for method in methods {
                handler_set[method as usize] = Some(func.clone());
            }
        } else {
            let mut handler_set = [None, None, None, None, None, None, None, None, None];
            for method in methods {
                handler_set[method as usize] = Some(func.clone());
            }
            self.handlers.insert(pattern, handler_set)?;
        }

        Ok(())
    }

    pub async fn run(&self, req: Request, env: Env) -> Result<Response> {
        if let Ok(Match { value, params }) = self.handlers.at(&req.path()) {
            if let Some(handler) = value[req.method() as usize].as_ref() {
                return match handler {
                    Handler::Sync(func) => (func)(req, env, params),
                    Handler::Async(func) => (func)(req, env, params).await,
                };
            }
            return Response::error("Method Not Allowed", 405);
        }
        Response::error("Not Found", 404)
    }
}

impl Default for Router<'_> {
    fn default() -> Self {
        Self {
            handlers: Node::new(),
        }
    }
}
