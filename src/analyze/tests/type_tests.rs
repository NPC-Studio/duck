use super::*;
use crate::{analyze::*, array, function, record, test_expr_type, test_var_type};
use Ty::*;

// Basic expressions
test_expr_type!(undefined, "undefined" => Undefined);
test_expr_type!(noone, "noone" => Noone);
test_expr_type!(bools, "true" => Bool, "false" => Bool);
test_expr_type!(real, "1" => Real);
test_expr_type!(real_float, "0.1" => Real);
test_expr_type!(hex, "$ffffff" => Real);
test_expr_type!(string, r#""foo""# => Str);
test_expr_type!(grouping, "(0)" => Real);
test_var_type!(postfix, "var x = 0, y = x++, z = x--;", x: Real, y: Real, z: Real);
test_var_type!(
    unary,
    "var a = 0, b = ++a, c = --a, d = +a, e = -a, f = ~a, g = true, h = !g;",
    a: Real,
    b: Real,
    c: Real,
    d: Real,
    e: Real,
    f: Real,
    g: Bool,
    h: Bool,
);
test_expr_type!(ternary, "true ? 0 : 0" => Real);
// test_expr_type!(
//     null_coalecence,
//     "function(x) {
//         return x ?? 0;
//     }" => new_function!((new_union!(Real, Undefined)) => Real)
// );
test_expr_type!(
    evaluation,
    "1 + 1" => Real,
    r#""foo" + "bar""# => Str,
    "1 * 1" => Real,
    "1 / 1" => Real,
    "1 % 1" => Real,
    "1 div 1" => Real,
);
test_expr_type!(logical, "true && false" => Bool);

// Local variable
test_var_type!(local_var, "var a = 0", a: Real);
test_var_type!(null_local_var, "var a;", a: Null);
test_var_type!(assign_to_null_var, "var a; a = 0;", a: Real);

// Globals
test_var_type!(globalvar, "globalvar foo;", foo: Null);
test_var_type!(globalvar_assign, "globalvar foo; foo = 0", foo: Real);
test_var_type!(global, "global.foo = 0;", foo: Real);

// Enums
test_expr_type!(enum_declaration, "enum foo { bar }" => record!(bar: Real));
test_var_type!(
    access_enum,
    "enum foo { bar }; 
    var bar = foo.bar;",
    bar: Real,
);

// Macros
test_var_type!(macro_reference, "#macro foo 0\nvar bar = foo;", bar: Any);

// Arrays
test_expr_type!(empty_array, "[]" => array!(Any));
test_expr_type!(constant_array, "[0]" => array!(Real));
test_expr_type!(nested_array, "[[[0]]]" => array!(array!(array!(Real))));
test_var_type!(array_access, "var x = [0], y = x[0];", y: Real);

// Structs
test_expr_type!(empty_struct, "{}" => record!());
test_expr_type!(populated_struct, "{ x: 0 }" => record!(x: Real));
test_var_type!(struct_access, "var foo = { x: 0 }, bar = foo.x;", bar: Real,);
test_var_type!(
    struct_extention,
    "var foo = { x: 0 };
    foo.y = 0;",
    foo: record!(x: Real, y: Real),
);
test_var_type!(
    nested_structs,
    "var foo = { x: { y: { z: 0 } } };
    var bar = foo.x.y.z;",
    bar: Real,
);
test_var_type!(
    struct_field_transfer,
    "var foo = { x: 0 };
    var bar = { y: 0 };
    foo.x = bar.y;",
    foo: record!(x: Real),
);
test_var_type!(
    function_on_struct,
    "var foo = {
        bar: function() { return 0; },
    };
    var fizz = foo.bar();",
    fizz: Real
);

// Functions
test_expr_type!(function, "function() {}" => function!(() => Undefined));
test_var_type!(named_function, "function foo() {};", foo: function!(() => Undefined));
test_var_type!(
    return_nothing,
    "var foo = function() {};
    var bar = foo();",
    bar: Undefined,
);
test_var_type!(
    return_constant,
    "var foo = function() { return 0; };
    var bar = foo();",
    bar: Real,
);
test_var_type!(
    return_inferred_constant,
    "var foo = function(x) { return x + 1; };
    var bar = foo(1);",
    bar: Real,
);
test_var_type!(
    identity_function,
    "var foo = function(x) { return x; };
    var bar = foo(true);",
    bar: Bool,
);
test_var_type!(
    return_generic_array,
    "var foo = function(x) { return x[0]; };
    var bar = foo([0]);",
    bar: Real,
);
test_var_type!(
    return_generic_struct,
    "var foo = function(x) { return x.y; };
    var bar = foo({ y: 0 });",
    bar: Real,
);
test_var_type!(
    return_other_function_return,
    "function wrapper(lambda) {
        return lambda(0);
    }
    function inner(n) { return n; }
    var data = wrapper(inner);",
    data: Real,
);
test_var_type!(
    return_advanced_generic,
    r#"var foo = function(a, b) {
        return a[b];
    }
    var bar = function(x, y) {
        return x + y * 2;
    }
    var fizz = foo(["hello"], 0);
    var buzz = foo([ { a: true } ], bar(1, 2));"#,
    fizz: Str,
    buzz: record!(a: Bool)
);
test_var_type!(
    multi_use_identity,
    "var foo = function(a) {
        return a;
    }
    var bar = foo(true);
    var fizz = foo(0);",
    bar: Bool,
    fizz: Real,
);
test_var_type!(
    return_onto_known_type,
    "var foo = function() {
        return 0;
    }
    var bar = 0;
    bar = foo();",
    bar: Real
);
// This one will require traits! (Returns<T> in particular)
test_expr_type!(
    infer_function_in_parameters,
    "function(x) { return x() + 1; }" => function!(
        (function!(() => Real)) => Real
    )
);
test_expr_type!(
    infer_array_in_parameters,
    "function(x) { return x[0] + 1; }" => function!(
        (array!(Real)) => Real
    )
);
test_expr_type!(
    infer_struct_in_parameters,
    "function(x) { return x.y + 1; }" => function!(
        (record!(y: Real)) => Real
    )
);
test_var_type!(
    mutate_struct_via_function,
    "var foo = function(a) {
        a.a = 0;
    }
    var bar = {};
    foo(bar);",
    bar: record!(a: Real)
);
test_var_type!(
    retain_all_fields_in_generic_call,
    "var foo = function(a) {
        a.a = 0;
        return a;
    }
    var bar = { a: 0, b: 0 };
    foo(bar);",
    bar: record!(a: Real, b: Real)
);
test_var_type!(
    retain_all_fields_in_generic_call_after_return,
    "var foo = function(a) {
        a.a = 0;
        return a;
    }
    var bar = { a: 0, b: 0 };
    bar = foo(bar);",
    bar: record!(a: Real, b: Real)
);

// Self
test_var_type!(self_assignment_no_keyword, "foo = 0;", foo: Real);
test_var_type!(self_assignment_with_keyword, "self.foo = 0;", foo: Real);
test_var_type!(
    function_write_constant_to_self,
    "self.a = 0;
    function bar() { self.a = 0; }",
    bar: function!(() => Undefined),
);
test_var_type!(
    function_write_parameter_to_self,
    "self.a = 0;
    function bar(x) { self.a = x + 1; }",
    bar: function!((Real) => Undefined),
);
test_var_type!(function_extend_self, "function foo() { self.a = 0; }", a: Real,);
test_var_type!(
    function_extend_self_nested,
    "function foo() { 
        function bar() { self.a = 0; }
    }",
    a: Real,
);
// test_var_type!(
//     bound_scope_in_struct,
//     "var foo = {
//         bar: 0,
//         fizz: function() {
//             return self.bar;
//         }
//     };
//     var buzz = foo.fizz();",
//     buzz: Real,
// );
test_var_type!(
    obj_setter,
    "self.x = 0;
    self.y = 0;
    function set(obj) {
        self.x = obj.x;
        self.y = obj.y;
    }",
    set: function!((record!(x: Real, y: Real)) => Undefined),
);

// Constructors
test_var_type!(
    constructor,
    "var foo = function() constructor {
        self.a = 0;
    }
    var bar = new foo();",
    bar: record!(a: Real)
);
test_var_type!(
    constructor_with_parameter,
    "function foo(y) constructor {
        self.x = y;
    }
    var bar = foo(0);",
    bar: record!(x: Real)
);
test_var_type!(
    constructor_getter,
    "var foo = function() constructor {
        self.a = 0;
        function get_a() {
            return self.a;
        }
    }
    var bar = new foo()
    var fizz = bar.get_a();",
    fizz: Real,
);
test_var_type!(
    inheritance,
    "var foo = function() constructor {
        self.a = 0;
    }
    var bar = function() : foo() constructor {}
    var fizz = new bar();",
    fizz: record!(a: Real)
);
test_var_type!(
    inheritance_passing_arguments,
    "var foo = function(x) constructor {
        self.a = x;
    }
    var bar = function(x) : foo(x) constructor {}
    var fizz = new bar(0);",
    fizz: record!(a: Real)
);
test_var_type!(
    multi_inheritance,
    "var foo = function() constructor {
        self.a = 0;
    }
    var bar = function() : foo() constructor {
        self.b = 0;
    }
    var fizz = function() : bar() constructor {}
    var buzz = new fizz();",
    buzz: record!(a: Real, b: Real)
);
test_var_type!(
    alias_function,
    "function foo() constructor {
        self.x = 0;
    }
    var bar = function() {
        var new_struct = new foo();
        return new_struct;
    }
    var fizz = bar();",
    fizz: record!(x: Real)
);
// test_var_type!(
//     constructor_clone,
//     "function foo() constructor {
//         function clone() { return new foo(); }
//     }
//     var bar = new foo();
//     var fizz = bar.clone();",
//     fizz: record!()
// );

// Out of order
// test_var_type!(
//     function_read_self_out_of_order,
//     "function bar() { return self.a; }
//     self.a = 0;",
//     bar: function!(() => Real),
// );
// test_var_type!(
//     function_write_self_out_of_order,
//     "function bar(x) { self.a = x; }
//     self.a = 0;",
//     bar: function!((Real) => Undefined),
// );
// test_var_type!(
//     function_calls_out_of_order,
//     "function foo() { self.bar();}
//     function bar() {}",
//     bar: function!(() => Undefined),
// );
// test_var_type!(
//     identity_out_of_order,
//     "function wrapper() {
//         return identity(0);
//     }
//     function identity(x) {
//         return x;
//     }
//     var bar = wrapper();",
//     bar: Real,
// );

// Stress tests
test_var_type!(
    complicted_data_construction,
    "var build_data = function(x, y, z) {
        return {
            x: x,
            y: y(0),
            z: z[0][0].a.b.c,
        };
    }
    var build_x = function(x) { return x; }
    var y_fn = function(n) { return n; }
    var z = [[{ a: { b: { c: 0 }}}]];
    var data = build_data(build_x(0), y_fn, z);
    var output = data.x + data.y + data.z;",
    z: array!(array!(record!(a: record!(b: record!(c: Real))))),
    data: record!(x: Real, y: Real, z: Real),
    output: Real
);
test_var_type!(
    vec_2,
    r#"
    function Vec2(_x, _y) constructor {
        self.x = _x;
        self.y = _y;

        static set = function(o) {
            self.x = o.x;
            self.y = o.y;
        }

        // static clone = function() {
        //     return Vec2(self.x, self.y);
        // }

        // static eq = function(o) {
        //     if o == undefined {
        //         return false;
        //     }
        //     return o.x == self.x && o.y == self.y;
        // }


    }

    var a = Vec2(0, 0);
    "#,
    a: record!(
        x: Real,
        y: Real,
        set: function!((record!(x: Real, y: Real)) => Undefined),
    )
);
