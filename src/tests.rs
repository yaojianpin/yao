use crate::{sch::Event, ActPlugin, Engine, Message, Workflow};
use rhai::plugin::*;
use std::sync::Arc;

#[tokio::test]
async fn engine_start() {
    let engine = Engine::new();

    let e = engine.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        e.close();
    });
    engine.start().await;
    assert!(true);
}

#[tokio::test]
async fn engine_start_async() {
    let engine = Engine::new();

    let e = engine.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        e.close();
    });

    tokio::spawn(async move {
        engine.start().await;
    });

    assert!(true);
}

#[tokio::test]
async fn engine_register_event() {
    let engine = Engine::new();
    engine.register_event(&Event::OnStart(Arc::new(|_w: &Workflow| {})));
    assert!(engine.evts().len() == 1);
}

#[tokio::test]
async fn engine_register_plugin() {
    let engine = Engine::new();

    let plugin_count = engine.plugins.lock().unwrap().len();
    engine.register_plugin(&TestPlugin::default());

    assert_eq!(engine.plugins.lock().unwrap().len(), plugin_count + 1);
}

#[tokio::test]
async fn engine_register_action() {
    let mut engine = Engine::new();

    let add = |a: i64, b: i64| Ok(a + b);
    let hash = engine.register_action("add", add);

    assert!(engine.action().contains_fn(hash));
}

#[tokio::test]
async fn engine_register_module() {
    let engine = Engine::new();

    let mut module = Module::new();
    combine_with_exported_module!(&mut module, "role", test_module);
    engine.register_module("test", &module);

    assert!(engine.modules().contains_key("test"));
}

#[tokio::test]
async fn engine_on_message() {
    let engine = Engine::new();
    let workflow = Workflow::new().with_job(|job| {
        job.with_step(|step| {
            step.with_subject(|sub| sub.with_matcher("any").with_users(r#"["a"]"#))
        })
    });

    println!("{}", workflow.to_string().unwrap());
    let e = engine.clone();
    engine.on_message(move |msg: &Message| {
        assert_eq!(msg.user, "a");
        e.close();
    });
    engine.push(&workflow);
    engine.start().await;
}

#[tokio::test]
async fn engine_builder() {
    let workflow = Workflow::new().with_name("w1").with_job(|job| {
        job.with_id("job1")
            .with_name("job 1")
            .with_env("v", 0.into())
            .with_step(|step| {
                step.with_id("step1")
                    .with_name("step1")
                    .with_run(r#"print("step1")"#)
                    .with_branch(|branch| {
                        branch
                            .with_if(r#"${ env.get("v") > 100 }"#)
                            .with_step(|step| step.with_name("step3").with_run(r#"print("step3")"#))
                    })
                    .with_branch(|branch| {
                        branch
                            .with_if(r#"${ env.get("v") <= 100 }"#)
                            .with_step(|step| step.with_name("step4").with_run(r#"print("step4")"#))
                    })
            })
            .with_step(|step| step.with_name("step2").with_run(r#"print("step2")"#))
    });

    assert_eq!(workflow.name, "w1");

    let job = workflow.job("job1").unwrap();
    assert_eq!(job.name, "job 1");
    assert_eq!(job.steps.len(), 2);

    let step = job.step("step1").unwrap();
    assert_eq!(step.name, "step1");
    assert_eq!(step.branches.len(), 2);
}

#[derive(Debug, Default, Clone)]
struct TestPlugin;

impl ActPlugin for TestPlugin {
    fn on_init(&self, _engine: &Engine) {
        println!("TestPlugin");
    }
}

#[export_module]
mod test_module {

    #[export_fn]
    pub fn test(_name: &str) {}
}
