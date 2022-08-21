# rules for single qubit gate, in SymbolicCircuit.jl


using Metatheory.Library: @right_associative, @left_associative

ra_rule = @right_associative (*)
la_rule = @left_associative (*)

commute_rule = @rule a b a::Gate * b::Gate => :($(b) * $(a)) where is_commute(a, b)
cancel_rule = @rule a b a::Gate * b::Gate => One() where is_cancel(a, b)
expand_rule = @rule a a::Gate => expand(a) where is_expand(a)
merge_rule = @rule a b a::Gate * b::Gate => merge(a, b) where is_merge(a, b)


"""could be used, but does not work well"""
# commute_rule = @rule a b a::Gate * b::Gate --> b * a where is_commute(a, b)
# cancel_rule = @rule a b a::Gate * b::Gate --> One() where is_cancel(a, b)
"""end"""


one_rules = @theory a b begin 
    b::One * a::Gate --> a
    a::Gate * b::One --> a
    b::One * a::Real --> a
    a::Real * b::One --> a
end

function get_simplify_rules()
    v = AbstractRule[]
    push!(v, ra_rule)
    push!(v, la_rule)
    push!(v, commute_rule)
    push!(v, cancel_rule)
    push!(v, expand_rule)
    push!(v, merge_rule)
    append!(v, one_rules)
    return v
end
