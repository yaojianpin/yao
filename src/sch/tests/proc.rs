use crate::{
    sch::{Proc, Scheduler},
    utils, Workflow,
};
use std::sync::Arc;

#[tokio::test]
async fn proc_messages() {
    let mut workflow = create_workflow();
    let id = utils::longid();
    let (proc, scher) = create_proc(&mut workflow, &id);
    scher.sched_proc(&proc);
    let tid = utils::shortid();
    let msg = proc.make_message(&tid, "u1");

    assert!(scher.message(&msg.id).is_some())
}

fn create_proc(workflow: &mut Workflow, id: &str) -> (Proc, Arc<Scheduler>) {
    let scher = Scheduler::new();

    workflow.set_biz_id(id);
    let proc = scher.create_raw_proc(workflow);

    scher.cache().push(&proc);
    (proc.clone(), Arc::new(scher))
}

fn create_workflow() -> Workflow {
    let text = include_str!("./simple.yml");
    let workflow = Workflow::from_str(text).unwrap();
    workflow
}
