use super::*;

#[test]
fn lazy_is_unstable() {
  Test::new()
    .justfile("set lazy\n\nfoo:\n  @echo hello")
    .arg("foo")
    .stderr_regex("error: The `lazy` setting is currently unstable\\..*")
    .status(1);
}

#[test]
fn lazy_unused_not_evaluated() {
  Test::new()
    .justfile(
      "
        set lazy

        unused := `exit 1`

        foo:
          @echo hello
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .arg("foo")
    .stdout("hello\n")
    .success();
}

#[test]
fn lazy_used_is_evaluated() {
  Test::new()
    .justfile(
      "
        set lazy

        used := `exit 1`

        @foo:
          echo foo
          echo {{used}}
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .stderr(
      "
        error: Backtick failed with exit code 1
         ——▶ justfile:3:9
          │
        3 │ used := `exit 1`
          │         ^^^^^^^^
      ",
    )
    .failure();
}

#[test]
fn eager_unused_evaluated() {
  Test::new()
    .justfile(
      "
        unused := `exit 1`

        foo:
          @echo hello
      ",
    )
    .arg("foo")
    .stderr(
      "
      error: Backtick failed with exit code 1
       ——▶ justfile:1:11
        │
      1 │ unused := `exit 1`
        │           ^^^^^^^^
      ",
    )
    .status(1);
}

#[test]
fn lazy_exports_always_evaluated() {
  Test::new()
    .justfile(
      "
        set lazy

        export X := 'exported'

        foo:
          @echo $X
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .arg("foo")
    .stdout("exported\n")
    .success();
}

#[test]
fn lazy_export_setting_evaluates_all() {
  Test::new()
    .justfile(
      "
        set lazy
        set export

        X := 'exported'

        foo:
          @echo $X
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .arg("foo")
    .stdout("exported\n")
    .success();
}

#[test]
fn lazy_submodule_unused_root() {
  Test::new()
    .justfile(
      "
        set lazy

        unused := `exit 1`

        mod sub
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .write("sub/mod.just", "foo:\n  @echo hello")
    .args(["sub", "foo"])
    .stdout("hello\n")
    .success();
}

#[test]
fn lazy_transitive_evaluated() {
  Test::new()
    .justfile(
      "
        set lazy

        a := b
        b := c
        c := 'value'
        unused := `exit 1`

        foo:
          @echo {{a}}
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .arg("foo")
    .stdout("value\n")
    .success();
}

#[test]
fn lazy_explicit_true() {
  Test::new()
    .justfile(
      "
        set lazy := true

        unused := `exit 1`

        foo:
          @echo hello
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .arg("foo")
    .stdout("hello\n")
    .success();
}

#[test]
fn lazy_explicit_false() {
  Test::new()
    .justfile(
      "
        set lazy := false

        unused := `exit 1`

        foo:
          @echo hello
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .arg("foo")
    .stderr(
      "
      error: Backtick failed with exit code 1
       ——▶ justfile:3:11
        │
      3 │ unused := `exit 1`
        │           ^^^^^^^^
      ",
    )
    .status(1);
}

#[test]
fn lazy_used_variable_evaluated() {
  Test::new()
    .justfile(
      "
        set lazy

        used := 'bar'
        unused := `exit 1`

        foo:
          @echo {{used}}
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .arg("foo")
    .stdout("bar\n")
    .success();
}

#[test]
fn lazy_dependency_variable_evaluated() {
  Test::new()
    .justfile(
      "
        set lazy

        x := 'dep_value'
        unused := `exit 1`

        foo: (bar x)

        bar val:
          @echo {{val}}
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .arg("foo")
    .stdout("dep_value\n")
    .success();
}

#[test]
fn lazy_parameter_default_evaluated() {
  Test::new()
    .justfile(
      "
        set lazy

        x := 'default'
        unused := `exit 1`

        foo val=x:
          @echo {{val}}
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .arg("foo")
    .stdout("default\n")
    .success();
}

#[test]
fn lazy_multiple_invocations_evaluate_union_of_needed_variables() {
  Test::new()
    .justfile(
      "
        set lazy

        x := 'foo'
        y := 'bar'
        unused := `exit 1`

        foo:
          @echo {{x}}

        bar:
          @echo {{y}}
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .args(["foo", "bar"])
    .stdout("foo\nbar\n")
    .success();
}

#[test]
fn lazy_root_unused_not_evaluated_for_submodule_recipe() {
  Test::new()
    .justfile(
      "
        set lazy

        unused := `exit 1`

        mod foo
      ",
    )
    .write(
      "foo.just",
      "bar:
  @echo ok
",
    )
    .env("JUST_UNSTABLE", "1")
    .args(["foo", "bar"])
    .stdout("ok\n")
    .success();
}

#[test]
fn lazy_submodule_unused_not_evaluated_for_root_recipe() {
  Test::new()
    .justfile(
      "
        set lazy

        foo:
          @echo ok

        mod bar
      ",
    )
    .write(
      "bar.just",
      "unused := `exit 1`

baz:
  @echo baz
",
    )
    .env("JUST_UNSTABLE", "1")
    .arg("foo")
    .stdout("ok\n")
    .success();
}

#[test]
fn lazy_same_name_recipes_in_sibling_modules_both_evaluate() {
  Test::new()
    .justfile(
      "
        set lazy

        mod foo
        mod bar
      ",
    )
    .write(
      "foo.just",
      "x := 'foo'
unused := `exit 1`

build:
  @echo {{x}}
",
    )
    .write(
      "bar.just",
      "y := 'bar'
unused := `exit 1`

build:
  @echo {{y}}
",
    )
    .env("JUST_UNSTABLE", "1")
    .args(["foo::build", "bar::build"])
    .stdout("foo\nbar\n")
    .success();
}

#[test]
fn lazy_same_name_recipe_in_other_module_is_not_evaluated() {
  Test::new()
    .justfile(
      "
        set lazy

        mod foo
        mod bar
      ",
    )
    .write(
      "foo.just",
      "x := 'foo'

build:
  @echo {{x}}
",
    )
    .write(
      "bar.just",
      "unused := `exit 1`

build:
  @echo bar
",
    )
    .env("JUST_UNSTABLE", "1")
    .arg("foo::build")
    .stdout("foo\n")
    .success();
}

#[test]
fn lazy_evaluate_single_does_not_evaluate_unrelated_variable() {
  Test::new()
    .args(["--evaluate", "foo"])
    .justfile(
      "
        set lazy

        foo := 'bar'
        unused := `exit 1`
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .stdout("bar")
    .success();
}

#[test]
fn lazy_evaluate_does_not_evaluate_private_unused_variable() {
  Test::new()
    .arg("--evaluate")
    .justfile(
      "
        set lazy

        foo := 'bar'

        [private]
        hidden := `exit 1`
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .stdout("foo := \"bar\"\n")
    .success();
}

#[test]
fn lazy_command_only_evaluates_exported_variables() {
  Test::new()
    .args(["--command", "sh", "-c", "printf $FOO"])
    .justfile(
      "
        set lazy

        export FOO := 'bar'
        unused := `exit 1`

        foo:
          @echo foo
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .stdout("bar")
    .success();
}

#[test]
fn lazy_choose_only_evaluates_chosen_recipe_variables() {
  Test::new()
    .arg("--choose")
    .env("JUST_CHOOSER", "head -n1")
    .justfile(
      "
        set lazy

        bar_value := 'bar'
        foo_value := `exit 1`

        foo:
          @echo {{foo_value}}

        bar:
          @echo {{bar_value}}
      ",
    )
    .env("JUST_UNSTABLE", "1")
    .stdout("bar\n")
    .success();
}
