# vec_to_binary_tree

<!-- toc GFM -->

* [Correct Implementation:](#correct-implementation)

<!-- toc -->

## Correct Implementation:

- https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/435296/How-do-I-create-the-TreeNode-from-an-array-of-numbers-using-javascript-or-java/

```typescript
export class MyNode {
  constructor(
    public val: number,
    public left: MyNode = null,
    public right: MyNode = null
  ) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}
export function buildTreeLeekCode(inputArray: number[]): MyNode {
  const root = new MyNode(inputArray.shift());
  const queue = [root];
  while (queue.length > 0 && inputArray.length > 0) {
    const curNode = queue.shift();
    const leftVal = inputArray.shift();
    const rightVal = inputArray.shift();
    if (leftVal !== null) {
      curNode.left = new MyNode(leftVal);
      queue.push(curNode.left);
    }
    if (rightVal !== null) {
      curNode.right = new MyNode(rightVal);
      queue.push(curNode.right);
    }
  }

  return root;
}
```
