pub fn rectangle_area(x1:f64,y1:f64,x2:f64,y2:f64)->f64{(x2-x1).abs()*(y2-y1).abs()}
pub fn distance(x1:f64,y1:f64,x2:f64,y2:f64)->f64{((x2-x1).powi(2)+(y2-y1).powi(2)).sqrt()}

pub fn grid_points(x1:f64,y1:f64,x2:f64,y2:f64,spacing:f64)->Vec<(f64,f64)>{
 let (minx,maxx)=(x1.min(x2),x1.max(x2));let(miny,maxy)=(y1.min(y2),y1.max(y2));let mut out=Vec::new();if spacing<=0.0{return out;}
 let mut y=miny+spacing/2.0;while y<maxy{let mut x=minx+spacing/2.0;while x<maxx{out.push((x,y));x+=spacing;}y+=spacing;}out
}

/// Repeatable pseudo-naturalistic layout: staggered rows plus deterministic jitter.
/// No RNG means regenerating the same area/spacing produces the same documentation.
pub fn naturalized_points(x1:f64,y1:f64,x2:f64,y2:f64,spacing:f64)->Vec<(f64,f64)>{
 let(minx,maxx)=(x1.min(x2),x1.max(x2));let(miny,maxy)=(y1.min(y2),y1.max(y2));let mut out=Vec::new();if spacing<=0.0{return out;}
 let mut row=0usize;let mut y=miny+spacing*0.55;while y<maxy{let off=if row%2==0{0.0}else{spacing*0.48};let mut col=0usize;let mut x=minx+spacing*0.55+off;while x<maxx{
  let jx=(((row*37+col*17)%11)as f64-5.0)*spacing*0.025;let jy=(((row*19+col*29)%13)as f64-6.0)*spacing*0.02;
  out.push(((x+jx).clamp(minx+spacing*0.2,maxx-spacing*0.2),(y+jy).clamp(miny+spacing*0.2,maxy-spacing*0.2)));col+=1;x+=spacing;
 }row+=1;y+=spacing*0.88;}out
}

#[cfg(test)]mod tests{use super::*;#[test]fn area(){assert_eq!(rectangle_area(0.0,0.0,10.0,5.0),50.0)}#[test]fn deterministic(){assert_eq!(naturalized_points(0.0,0.0,20.0,10.0,3.0),naturalized_points(0.0,0.0,20.0,10.0,3.0));}}
