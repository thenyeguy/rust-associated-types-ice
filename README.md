I was working on a complicated generic container, when I started running into an
internal compiler error in rust.

I reduced it to the following relevant bits:
 * A TestTrait with two associated types in one module.
 * A type in a separate module that wraps the TestTrait in a Box.
 * An main function that includes the library and tries to create a variable of
   that type.

Running `cargo build --lib` successfully produces a rust library file.

Moving the TestTrait into the same file as the type declaration fixes the ICE.

Commenting out the variable declaration in the main function fixes the ICE.

## Rust Version
     >> rustc --version --verbose
    rustc 1.1.0-nightly (e5394240a 2015-05-14) (built 2015-05-13)
    binary: rustc
    commit-hash: e5394240a295650b567aa406b4a0e1e3a6749a5f
    commit-date: 2015-05-14
    build-date: 2015-05-13
    host: x86_64-apple-darwin
    release: 1.1.0-nightly

## Backtrace
     >> RUST_BACKTRACE=1 cargo build
       Compiling minimalice v0.1.0 (file:///Users/mnye/Files/Programming/minimalice)
    error: internal compiler error: unexpected panic
    note: the compiler unexpectedly panicked. this is a bug.
    note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
    note: run with `RUST_BACKTRACE=1` for a backtrace
    thread 'rustc' panicked at 'assertion failed: bound_list_is_sorted(&bounds.projection_bounds)', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librustc
    /middle/ty.rs:3237

    stack backtrace:
       1:        0x10c75233f - sys::backtrace::write::h4c9c72fd000cd0edxes
       2:        0x10c75ae30 - panicking::on_panic::h63ca458201c7b8b6Kuw
       3:        0x10c714c15 - rt::unwind::begin_unwind_inner::h0e18b375a5759439tcw
       4:        0x109772eff - rt::unwind::begin_unwind::h13174421819739485657
       5:        0x109862e99 - middle::ty::mk_trait::h6ecf9908820f3874Ta5
       6:        0x109aee631 - metadata::tydecode::parse_ty_::h11720362305860207704
       7:        0x109aed7a8 - metadata::tydecode::parse_ty_::h11720362305860207704
       8:        0x109aef5c3 - metadata::tydecode::parse_substs_::h16912297305823193881
       9:        0x109aee22f - metadata::tydecode::parse_ty_::h11720362305860207704
      10:        0x109afcad7 - metadata::decoder::get_type::h22e10dcd21be5e70cNj
      11:        0x10994ff28 - middle::ty::lookup_item_type::heeefff8c4b47066c8s8
      12:        0x10956126d - check::FnCtxt<'a, 'tcx>.AstConv<'tcx>::get_item_type_scheme::h027252bb782a5186iwo
      13:        0x1095e3509 - astconv::ast_path_to_ty::h3b543c7c3d9b47e562u
      14:        0x1095cff2c - astconv::finish_resolving_def_to_ty::h0072cddef905eccbkIv
      15:        0x10958c7f9 - astconv::ast_ty_to_ty::h9d52fda6176753468Jv
      16:        0x10957d2e9 - check::GatherLocalsVisitor<'a, 'tcx>.Visitor<'tcx>::visit_local::h7f6b1cb8f47b97a30xn
      17:        0x109562605 - check::check_fn::h674c1b1a7b5bf46fNEn
      18:        0x1095796a3 - check::check_bare_fn::h6e3aca4af8072fe6qun
      19:        0x109577739 - check::CheckItemBodiesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::hf4a373fffeb76a8dtrn
      20:        0x10963ca8a - check_crate::closure.38894
      21:        0x109636337 - check_crate::h16de0a53da10556eZIC
      22:        0x108e8d347 - driver::phase_3_run_analysis_passes::h3e778c3265449835tGa
      23:        0x108e6efc3 - driver::compile_input::h24d152d79aa77f09Qba
      24:        0x108f306c3 - run_compiler::hed43454bb7f2bc6975b
      25:        0x108f2de2a - boxed::F.FnBox<A>::call_box::h17356779411131551354
      26:        0x108f2d387 - rt::unwind::try::try_fn::h3117498951650229503
      27:        0x10c7e3198 - rust_try_inner
      28:        0x10c7e3185 - rust_try
      29:        0x108f2d65d - boxed::F.FnBox<A>::call_box::h1499661684949148773
      30:        0x10c75982d - sys::thread::Thread::new::thread_start::h34a82086280644f7sxv
      31:     0x7fff88687267 - _pthread_body
      32:     0x7fff886871e4 - _pthread_start
