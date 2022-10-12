# Ownership and borrow checker

## TODO
+ Case with fn length(self) -> f64 : move occurs, the number is not usable anymore


```rust
{{#rustdoc_include rustviz_assets/example1/source.rs}}
```
<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display">
    <object type="image/svg+xml" class="copy code_panel" data="rustviz_assets/example1/vis_code.svg"></object>
    <object type="image/svg+xml" class="copy tl_panel" data="rustviz_assets/example1/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('copy')"></object>
</div>
