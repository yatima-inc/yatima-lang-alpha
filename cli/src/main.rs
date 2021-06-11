use std::{
  rc::Rc,
  path::PathBuf,
};

use structopt::StructOpt;
use yatima_utils::{
  file,
  store::{
    show,
    Store,
  },
};
use yatima_cli::{
  ipfs,
  repl,
  file::store::FileStore,
};
use yatima_core::{
  parse,
  name::Name,
};

#[derive(Debug, StructOpt)]
#[structopt(about = "A programming language for the decentralized web")]
enum Cli {
  Parse {
    /// Pin data to the local IPFS daemon
    #[structopt(short, long)]
    no_ipfs: bool,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
  },
  Check {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
  },
  Show {
    input: String,
    #[structopt(name = "type", default_value = "raw", long, short)]
    typ_: String,
  },
  Run {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
  },
  Repl,
}
//   Test,
#[tokio::main]
async fn main() -> std::io::Result<()> {
  let command = Cli::from_args();
  match command {
    Cli::Repl => {
      repl::main();
      Ok(())
    }
    Cli::Show { input, typ_ } => {
      let (_, link) = parse::package::parse_link(parse::span::Span::new(&input))
        .expect("Invalid links");
      let store = Rc::new(FileStore::new());
      match show(store, link, typ_) {
        Ok(s) => {
          println!("{}", s);
          Ok(())
        },
        Err(s) => {
          eprintln!("{}", s);
          Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        }
      }
    }
    Cli::Parse { no_ipfs, path } => {
      let root = std::env::current_dir()?;
      let store = Rc::new(FileStore::new());
      let env = file::parse::PackageEnv::new(root, path, store.clone());
      let (cid, p, d) = file::parse::parse_file(env);
      store.put(p.to_ipld());

      let ipld_cid = 
        if !no_ipfs {
          ipfs::dag_put(p.to_ipld()).await.expect("Failed to put to ipfs.")
        } else {
          "Not using ipfs".to_string()
        };
      println!("Package parsed:\n{} ipld_cid={}", cid, ipld_cid);
      println!("{}", d);
      Ok(())
    }
    Cli::Check { path } => {
      let store = Rc::new(FileStore::new());
      file::check_all_in_file(path, store)?;
      Ok(())
    }
    Cli::Run { path } => {
      let root = std::env::current_dir()?;
      let store = Rc::new(FileStore::new());
      let env = file::parse::PackageEnv::new(root, path.clone(), store.clone());
      let (_, p, defs) = file::parse::parse_file(env);
      let _cid = store.put(p.to_ipld());
      let _ipld_cid =
        ipfs::dag_put(p.to_ipld()).await.expect("Failed to put to ipfs.");
      let def = defs.get(&Name::from("main")).expect(&format!(
        "No `main` expression in package {} from file {:?}",
        p.name, path
      ));
      let mut dag = yatima_core::dag::DAG::from_term(&def.to_owned().term);
      dag.norm(&defs);
      println!("{}", dag);
      Ok(())
    }
  }
}

// for valgrind testing
// Cli::Test => {
//  use im::HashMap;
//  use yatima::{
//    core::dag::DAG,
//    parse::span::Span,
//  };
//  pub fn parse(
//    i: &str,
//  ) -> nom::IResult<Span, DAG, crate::parse::error::ParseError<Span>>
//  {
//    let (i, tree) = crate::parse::term::parse(i)?;
//    let (i, _) = nom::character::complete::multispace0(i)?;
//    let (i, _) = nom::combinator::eof(i)?;
//    let dag = DAG::from_term(&tree);
//    Ok((i, dag))
//  }
//  fn norm_assert(input: &str, result: &str) {
//    match parse(&input) {
//      Ok((_, mut dag)) => {
//        dag.norm(&HashMap::new());
//        assert_eq!(format!("{}", dag), result)
//      }
//      Err(_) => panic!("Did not parse."),
//    }
//  }
//  norm_assert(
//    "∀ (f: ∀ (A: Type) (x: A) -> Type) -> Type",
//    "∀ (f: ∀ (A: Type) (x: A) -> Type) -> Type",
//  );
//  // norm_assert("let f (A: Type) (x: A): Type = A; f", "λ A x => A");
//}
