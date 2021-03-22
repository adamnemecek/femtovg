# TODO

* does webgpu need a pseudotexture?
*

# states
* convex_fill
    * 

* concave_fill
    * state1:
        * no cull mode
        * rps: stencil_only_pipeline_state + fill_shape_stencil_state + * triangle topology
    * state2nonzero
        * ps: pipeline_state
        * stencil: fill_anti_alias_stencil_state_nonzero
        * topology: triangle strip
        * cull: back
        * default_stencil_state
    * state2evenodd
        * ps: pipelinestate
        * topology: triangle strip
        * cull: back
        * pipelinestate + fill_anti_alias_stencil_state_evenodd
        * default_stencil_state

* stroke
    * ps:
    * topology: trianglestrip
* stencil_stroke
    * ps: 
* clear_rect
    * 



* stencil_stroke
