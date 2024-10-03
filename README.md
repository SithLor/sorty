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
    #[inline(always)]
    pub fn midpoint(x1: f64, x2: f64, y1: f64, y2: f64) -> (f64, f64) {
        static ONE_HALF: f64 = 0.5;
        return ((x1 + x2) * ONE_HALF, (y1 + y2) * ONE_HALF);
    }

    #[inline(always)]
    pub fn midpoint_static(x1: f64, x2: f64, y1: f64, y2: f64, return_1: &mut f64, return_2: &mut f64) {
        static ONE_HALF: f64 = 0.5;
        *return_1 = (x1 + x2) * ONE_HALF;
        *return_2 = (y1 + y2) * ONE_HALF;
    }
}

mod test {
    use super::*;
    pub fn core_midpoint() {
        let timer = std::time::Instant::now();
        let mid = core::midpoint(3.324, -4532.432, 432.432, 334.4324);
        let elapsed: std::time::Duration = timer.elapsed();
        println!("core_midpoint {:?}ns", elapsed.as_nanos());
    }

    pub fn core_midpoint_static() {
        let timer = std::time::Instant::now();

        static mut X1: f64 = 3.324;
        static mut X2: f64 = -4532.432;
        static mut Y1: f64 = 432.432;
        static mut Y2: f64 = 334.4324;

        static mut RETURN_1: f64 = 0.0;
        static mut RETURN_2: f64 = 0.0;

        unsafe {
            core::midpoint_static(X1, X2, Y1, Y2, &mut RETURN_1, &mut RETURN_2);
        }

        let elapsed: std::time::Duration = timer.elapsed();
        unsafe {
            println!("core_midpoint_static {:?}ns, result: ({}, {})", elapsed.as_nanos(), RETURN_1, RETURN_2);
        }
    }
}

fn main() {
    test::core_midpoint();
    test::core_midpoint_static();
}