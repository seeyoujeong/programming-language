fn main() {
    println!("variable bindings");
    variable_bindings();
    println!("-------------------------");

    println!("scope and shadowing");
    scope_and_shadowing();
    println!("-------------------------");

    println!("declare first");
    declare_first();
    println!("-------------------------");

    println!("freezing");
    freezing();
    println!("-------------------------");
}

fn variable_bindings() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();
    let copied_integer = an_integer;

    println!("An integer: {}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings.
    // This warning can be silenced by prefixing the variable name with an underscore.
    let _unused_variable = 3u32;
}

fn scope_and_shadowing() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }

    println!("outside inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    
    println!("shadowed in outer block: {}", shadowed_binding);
}

fn declare_first() {
    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);
}

fn freezing() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable '_mutable_integer'
        let _mutable_integer = _mutable_integer;

        // Error! '_mutable_integer' is frozen in this scope
        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}
