mod core {

    // RUST: 1.81.0 nightly
    // link:https://godbolt.org/#g:!((g:!((g:!((h:codeEditor,i:(filename:'1',fontScale:14,fontUsePx:'0',j:1,lang:rust,selection:(endColumn:12,endLineNumber:4,positionColumn:12,positionLineNumber:4,selectionStartColumn:12,selectionStartLineNumber:4,startColumn:12,startLineNumber:4),source:'const+ONE_HALF:+f64+%3D+0.5%3B%0A%0A//(p1.x+%2B+p2.x),+(p1.y+%2B+p2.y)%0A%23%5Bno_mangle%5D%0Apub+fn+midpoint(x1:f64,x2:f64,y1:f64,y2:f64)+-%3E+(f64,f64)+%7B%0A++++return+((x1+%2B+x2)/ONE_HALF,+(y1+%2B+y2)/ONE_HALF)%3B%0A%7D'),l:'5',n:'1',o:'Rust+source+%231',t:'0')),k:30.58026563501256,l:'4',n:'0',o:'',s:0,t:'0'),(g:!((h:compiler,i:(compiler:nightly,filters:(b:'0',binary:'1',binaryObject:'1',commentOnly:'0',debugCalls:'1',demangle:'0',directives:'0',execute:'1',intel:'0',libraryCode:'0',trim:'1',verboseDemangling:'0'),flagsViewOpen:'1',fontScale:14,fontUsePx:'0',j:1,lang:rust,libs:!(),options:'-Z+tune-cpu%3Dmachine+-C+opt-level%3D3+-C+overflow-checks%3Dtrue+-C+strip%3Ddebuginfo+-C+target-cpu%3Dnative',overrides:!(),selection:(endColumn:15,endLineNumber:2,positionColumn:15,positionLineNumber:2,selectionStartColumn:9,selectionStartLineNumber:2,startColumn:9,startLineNumber:2),source:1),l:'5',n:'0',o:'+rustc+nightly+(Editor+%231)',t:'0')),k:36.08640103165411,l:'4',n:'0',o:'',s:0,t:'0'),(g:!((h:output,i:(compilerName:'rustc+1.81.0',editorid:1,fontScale:14,fontUsePx:'0',j:1,wrap:'1'),l:'5',n:'0',o:'Output+of+rustc+nightly+(Compiler+%231)',t:'0')),k:33.33333333333333,l:'4',n:'0',o:'',s:0,t:'0')),l:'2',n:'0',o:'',t:'0')),version:4
    // Build args: -Z tune-cpu=machine -C opt-level=3 -C overflow-checks=true -C strip=debuginfo -C target-cpu=native
    // X86_64 assembly output:
    //       midpoint:
    //      1     vaddsd  xmm0, xmm0, xmm1
    //      2     vaddsd  xmm0, xmm0, xmm0
    //      3     vaddsd  xmm1, xmm2, xmm3
    //      4     vaddsd  xmm1, xmm1, xmm1
    //      5     ret

    /// Computes the midpoint of two points `(x1, y1)` and `(x2, y2)`.
    ///
    /// # Arguments
    ///
    /// * `x1` - The x-coordinate of the first point.
    /// * `x2` - The x-coordinate of the second point.
    /// * `y1` - The y-coordinate of the first point.
    /// * `y2` - The y-coordinate of the second point.
    ///
    /// # Returns
    ///
    /// A tuple `(f64, f64)` representing the midpoint of the two points.
    ///
    /// # Examples
    ///
    /// ```
    /// let (x1, y1) = (3.324, 432.432);
    /// let (x2, y2) = (-4532.432, 334.4324);
    /// let midpoint = midpoint(x1, x2, y1, y2);
    /// assert_eq!(midpoint, (-2264.554, 383.4322));
    /// ```
    /// Performance: 30.58026563501256 nanoseconds
    #[inline(always)]
    pub fn midpoint(x1: f64, x2: f64, y1: f64, y2: f64) -> (f64, f64) {
        static ONE_HALF: f64 = 0.5;
        ((x1 + x2) * ONE_HALF, (y1 + y2) * ONE_HALF)
    }

    #[inline(always)]
    pub fn midpoint_static(
        x1: f64,
        x2: f64,
        y1: f64,
        y2: f64,
        return_1: &mut f64,
        return_2: &mut f64,
    ) {
        static ONE_HALF: f64 = 0.5;
        *return_1 = (x1 + x2) * ONE_HALF;
        *return_2 = (y1 + y2) * ONE_HALF;
    }
}
//test code

fn main() {

}
