<script>
    export const createWirelessEdge = (params, edgeId) => {
        return {
            id: edgeId,
            ...params,
            type: 'straight',
            animated: false,
            style: {
                stroke: 'var(--secondary-color)',
                strokeDasharray: '8 8',
                strokeWidth: 8,
            },
        };
    };
</script>
